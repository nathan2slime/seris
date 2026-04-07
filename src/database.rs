//! SQLite-backed persistence for lightweight bot state.

use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::Mutex,
    time::Duration,
};

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serenity::all::UserId;

use crate::types::{Error, SerisError};

/// Summary of a user's persisted command usage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandUsageSummary {
    /// Total number of recorded command uses.
    pub total_uses: i64,
    /// Number of distinct commands used.
    pub distinct_commands: i64,
    /// Most used command, if any.
    pub favorite_command: Option<String>,
    /// Usage count for the most used command.
    pub favorite_count: i64,
}

/// SQLite-backed persistence layer.
pub struct Database {
    connection: Mutex<Connection>,
}

impl Database {
    /// Opens the default database path.
    pub fn open_default() -> Result<Self, Error> {
        let path = default_database_path().ok_or(SerisError::InvalidConfig {
            field: "database path",
            reason: "could not be determined",
        })?;

        Self::open(path)
    }

    /// Opens a database at a specific path.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let connection = Connection::open(path)?;
        connection.busy_timeout(Duration::from_secs(2))?;
        connection.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            CREATE TABLE IF NOT EXISTS command_usage (
                user_id TEXT NOT NULL,
                command TEXT NOT NULL,
                count INTEGER NOT NULL DEFAULT 1,
                last_used INTEGER NOT NULL,
                PRIMARY KEY (user_id, command)
            );
            "#,
        )?;

        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    /// Records that a user used a command.
    pub fn record_command_usage(
        &self,
        user_id: UserId,
        command: &'static str,
    ) -> Result<(), Error> {
        let user_id = user_id.get();
        let now = Utc::now().timestamp();
        let connection = self.connection.lock().expect("database mutex");

        connection.execute(
            r#"
            INSERT INTO command_usage (user_id, command, count, last_used)
            VALUES (?1, ?2, 1, ?3)
            ON CONFLICT(user_id, command) DO UPDATE SET
                count = count + 1,
                last_used = excluded.last_used
            "#,
            params![user_id.to_string(), command, now],
        )?;

        Ok(())
    }

    /// Records command usage without failing the command if persistence is unavailable.
    pub fn record_command_usage_best_effort(&self, user_id: UserId, command: &'static str) {
        if let Err(err) = self.record_command_usage(user_id, command) {
            log::warn!("failed to persist command usage for /{command}: {err}");
        }
    }

    /// Returns a summary of a user's persisted command usage.
    pub fn command_usage_summary(&self, user_id: UserId) -> Result<CommandUsageSummary, Error> {
        let user_id = user_id.get();
        let connection = self.connection.lock().expect("database mutex");

        let (total_uses, distinct_commands) = connection.query_row(
            r#"
            SELECT COALESCE(SUM(count), 0), COUNT(*)
            FROM command_usage
            WHERE user_id = ?1
            "#,
            params![user_id.to_string()],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
        )?;

        let favorite = connection
            .query_row(
                r#"
                SELECT command, count
                FROM command_usage
                WHERE user_id = ?1
                ORDER BY count DESC, last_used DESC, command ASC
                LIMIT 1
                "#,
                params![user_id.to_string()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
            )
            .optional()?;

        let (favorite_command, favorite_count) = favorite
            .map(|(command, count)| (Some(command), count))
            .unwrap_or((None, 0));

        Ok(CommandUsageSummary {
            total_uses,
            distinct_commands,
            favorite_command,
            favorite_count,
        })
    }
}

fn default_database_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("SERIS_DB_FILE") {
        return Some(PathBuf::from(path));
    }

    if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
        return Some(
            PathBuf::from(xdg_data_home)
                .join("seris")
                .join("seris.sqlite3"),
        );
    }

    env::var("HOME").ok().map(|home| {
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("seris")
            .join("seris.sqlite3")
    })
}

#[cfg(test)]
mod tests {
    use super::Database;
    use serenity::all::UserId;
    use tempfile::tempdir;

    #[test]
    fn records_command_usage_and_summaries_persist() {
        let dir = tempdir().expect("temp dir");
        let db_path = dir.path().join("seris.sqlite3");
        let database = Database::open(&db_path).expect("database");
        let user_id = UserId::new(42);

        database
            .record_command_usage(user_id, "ping")
            .expect("first usage");
        database
            .record_command_usage(user_id, "ping")
            .expect("second usage");
        database
            .record_command_usage(user_id, "uptime")
            .expect("different command");

        let summary = database.command_usage_summary(user_id).expect("summary");

        assert_eq!(summary.total_uses, 3);
        assert_eq!(summary.distinct_commands, 2);
        assert_eq!(summary.favorite_command.as_deref(), Some("ping"));
        assert_eq!(summary.favorite_count, 2);
    }
}
