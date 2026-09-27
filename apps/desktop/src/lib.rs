//! Machie desktop shell.
//!
//! The shell wires the Tauri runtime to the Machie Core. It is a
//! presentation boundary only — no routing, permission, or
//! provider-selection logic lives here (see ADR-0001, spec Part P).
//!
//! Phase 0: the shell opens a window hosting the React UI. No user-facing
//! features are implemented yet.

/// Launch the Machie desktop application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running Machie desktop application");
}
