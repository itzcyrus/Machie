//! Versioned TOML configuration.
//!
//! Every persisted config file carries an explicit `config_version`. Any
//! change to the shape of persisted config requires a migration function
//! registered in [`migrations`] — mirrors the database migration rule in
//! the specification (Parts Q.5 and K.5).

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Current config schema version written by this build.
pub const CURRENT_CONFIG_VERSION: u32 = 1;

/// Root configuration structure persisted as TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachieConfig {
    /// Schema version. Must match [`CURRENT_CONFIG_VERSION`] after loading.
    pub config_version: u32,

    /// Network policy. See Part D.2 of the spec.
    #[serde(default)]
    pub network: NetworkConfig,
}

/// Network policy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Explicit network mode. Never silently changed at runtime.
    pub mode: NetworkMode,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            mode: NetworkMode::LocalOnly,
        }
    }
}

/// Explicit, always-visible network modes (spec Part D.2).
///
/// Web access and cloud inference are separate permissions — the four modes
/// below cover the valid combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkMode {
    /// No web, no cloud, no remote uploads.
    LocalOnly,
    /// Web search/fetch allowed. Cloud inference still off.
    WebEnabled,
    /// Configured gateway/direct inference allowed. Web may remain off.
    CloudEnabled,
    /// Web + cloud both enabled.
    Full,
}

impl Default for MachieConfig {
    fn default() -> Self {
        Self {
            config_version: CURRENT_CONFIG_VERSION,
            network: NetworkConfig::default(),
        }
    }
}

/// Errors that can occur when loading, migrating, or saving configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to parse config TOML: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("failed to serialize config TOML: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("config version {found} is newer than this build supports ({supported})")]
    VersionTooNew { found: u32, supported: u32 },

    #[error("no config migration registered from version {0}")]
    NoMigration(u32),

    #[error("config migration from version {from} did not advance config_version")]
    MigrationStalled { from: u32 },
}

impl MachieConfig {
    /// Parse a config from TOML text, applying any registered migrations.
    pub fn from_toml_str(text: &str) -> Result<Self, ConfigError> {
        let mut value: MachieConfig = toml::from_str(text)?;
        value.migrate()?;
        Ok(value)
    }

    /// Serialize this config to TOML text.
    pub fn to_toml_string(&self) -> Result<String, ConfigError> {
        Ok(toml::to_string_pretty(self)?)
    }

    /// Apply registered migrations until the config is at the current version.
    fn migrate(&mut self) -> Result<(), ConfigError> {
        if self.config_version > CURRENT_CONFIG_VERSION {
            return Err(ConfigError::VersionTooNew {
                found: self.config_version,
                supported: CURRENT_CONFIG_VERSION,
            });
        }
        while self.config_version < CURRENT_CONFIG_VERSION {
            let from = self.config_version;
            migrations::apply(from, self)?;
            if self.config_version == from {
                return Err(ConfigError::MigrationStalled { from });
            }
        }
        Ok(())
    }
}

/// Registry of config-shape migrations.
///
/// Phase 0: no migrations exist yet. The registry and its calling convention
/// are established now so that every future shape change has an obvious home.
///
/// A migration from `from_version` must leave `config.config_version` equal
/// to `from_version + 1` on success; otherwise the loader reports a stalled
/// migration rather than looping.
pub mod migrations {
    use super::{ConfigError, MachieConfig};

    /// Apply the migration from `from_version` to `from_version + 1`.
    pub fn apply(from_version: u32, _config: &mut MachieConfig) -> Result<(), ConfigError> {
        // No migrations registered yet. When adding one, replace this body
        // with a `match from_version` that dispatches to the correct step.
        Err(ConfigError::NoMigration(from_version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_roundtrips_through_toml() {
        let cfg = MachieConfig::default();
        let text = cfg.to_toml_string().expect("serialize");
        let parsed = MachieConfig::from_toml_str(&text).expect("parse");
        assert_eq!(parsed.config_version, CURRENT_CONFIG_VERSION);
        assert_eq!(parsed.network.mode, NetworkMode::LocalOnly);
    }

    #[test]
    fn rejects_future_config_version() {
        let text = r#"
            config_version = 999
            [network]
            mode = "local_only"
        "#;
        let err = MachieConfig::from_toml_str(text).unwrap_err();
        assert!(matches!(err, ConfigError::VersionTooNew { .. }));
    }

    #[test]
    fn defaults_network_mode_to_local_only() {
        let text = r#"config_version = 1"#;
        let cfg = MachieConfig::from_toml_str(text).expect("parse");
        assert_eq!(cfg.network.mode, NetworkMode::LocalOnly);
    }

    #[test]
    fn parses_each_network_mode() {
        for (raw, expected) in [
            ("local_only", NetworkMode::LocalOnly),
            ("web_enabled", NetworkMode::WebEnabled),
            ("cloud_enabled", NetworkMode::CloudEnabled),
            ("full", NetworkMode::Full),
        ] {
            let text = format!(
                "config_version = 1\n[network]\nmode = \"{raw}\"\n"
            );
            let cfg = MachieConfig::from_toml_str(&text).expect("parse");
            assert_eq!(cfg.network.mode, expected);
        }
    }
}