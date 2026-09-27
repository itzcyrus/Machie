//! SQLite connection pool and migration entry point.

use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use thiserror::Error;

/// Errors returned by [`connect`].
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("failed to open SQLite database at {path}: {source}")]
    Open {
        path: String,
        #[source]
        source: sqlx::Error,
    },

    #[error("failed to run database migrations: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

/// Open (creating if necessary) a SQLite pool at `path` and run pending
/// migrations.
///
/// Migrations are embedded at compile time from `crates/database/migrations`.
pub async fn connect(path: impl AsRef<Path>) -> Result<SqlitePool, DatabaseError> {
    let path = path.as_ref();

    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|source| DatabaseError::Open {
            path: path.display().to_string(),
            source,
        })?;

    let migrator = sqlx::migrate!("./migrations");
    migrator.run(&pool).await?;

    Ok(pool)
}
