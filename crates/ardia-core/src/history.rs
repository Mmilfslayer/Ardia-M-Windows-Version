use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::error::CoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HistoryId(pub Uuid);

impl HistoryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for HistoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: HistoryId,
    pub url: String,
    pub title: String,
    pub visit_count: u64,
    pub last_visit_time: DateTime<Utc>,
    pub typed_count: u64,
    pub profile_id: String,
    pub transition_type: TransitionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionType {
    Link,
    Typed,
    AutoBookmark,
    AutoSubframe,
    ManualSubframe,
    Generated,
    StartPage,
    FormSubmit,
    Reload,
    Keyword,
    KeywordGenerated,
}

pub struct HistoryManager {
    conn: Arc<RwLock<Connection>>,
    db_path: PathBuf,
}

impl HistoryManager {
    pub async fn new() -> CoreResult<Self> {
        let db_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ardia")
            .join("history.db");

        std::fs::create_dir_all(db_path.parent().unwrap())?;

        let conn = Connection::open(&db_path)?;

        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                visit_count INTEGER NOT NULL DEFAULT 0,
                last_visit_time TEXT NOT NULL,
                typed_count INTEGER NOT NULL DEFAULT 0,
                profile_id TEXT NOT NULL,
                transition_type TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_url ON history(url)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_profile ON history(profile_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_last_visit ON history(last_visit_time)",
            [],
        )?;

        // Enable FTS5 for full-text search
        conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS history_fts USING fts5(
                url, title, content='history', content_rowid='rowid'
            )",
            [],
        )?;

        conn.execute(
            "CREATE TRIGGER IF NOT EXISTS history_ai AFTER INSERT ON history BEGIN
                INSERT INTO history_fts(rowid, url, title) VALUES (new.rowid, new.url, new.title);
            END",
            [],
        )?;

        conn.execute(
            "CREATE TRIGGER IF NOT EXISTS history_ad AFTER DELETE ON history BEGIN
                INSERT INTO history_fts(history_fts, rowid, url, title) VALUES('delete', old.rowid, old.url, old.title);
            END",
            [],
        )?;

        conn.execute(
            "CREATE TRIGGER IF NOT EXISTS history_au AFTER UPDATE ON history BEGIN
                INSERT INTO history_fts(history_fts, rowid, url, title) VALUES('delete', old.rowid, old.url, old.title);
                INSERT INTO history_fts(rowid, url, title) VALUES (new.rowid, new.url, new.title);
            END",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(RwLock::new(conn)),
            db_path,
        })
    }

    pub async fn add_visit(
        &self,
        url: String,
        title: String,
        profile_id: String,
        transition_type: TransitionType,
        is_typed: bool,
    ) -> CoreResult<HistoryId> {
        let conn = self.conn.read().await;

        // Check if URL already exists
        let existing: Option<(String, u64, u64)> = conn
            .query_row(
                "SELECT id, visit_count, typed_count FROM history WHERE url = ?1 AND profile_id = ?2",
                params![&url, &profile_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()?;

        let now = Utc::now();
        let id = if let Some((existing_id, visit_count, typed_count)) = existing {
            let new_visit_count = visit_count + 1;
            let new_typed_count = typed_count + if is_typed { 1 } else { 0 };

            conn.execute(
                "UPDATE history SET visit_count = ?1, last_visit_time = ?2, typed_count = ?3, title = ?4 WHERE id = ?5",
                params![new_visit_count, now.to_rfc3339(), new_typed_count, &title, &existing_id],
            )?;

            HistoryId(Uuid::parse_str(&existing_id).unwrap_or_else(|_| Uuid::new_v4()))
        } else {
            let id = HistoryId::new();
            let typed_count = if is_typed { 1 } else { 0 };

            conn.execute(
                "INSERT INTO history (id, url, title, visit_count, last_visit_time, typed_count, profile_id, transition_type)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    id.to_string(),
                    &url,
                    &title,
                    1,
                    now.to_rfc3339(),
                    typed_count,
                    &profile_id,
                    format!("{:?}", transition_type),
                ],
            )?;

            id
        };

        Ok(id)
    }

    pub async fn get_history(
        &self,
        profile_id: &str,
        limit: usize,
        offset: usize,
    ) -> CoreResult<Vec<HistoryEntry>> {
        let conn = self.conn.read().await;

        let mut stmt = conn.prepare(
            "SELECT id, url, title, visit_count, last_visit_time, typed_count, profile_id, transition_type
             FROM history
             WHERE profile_id = ?1
             ORDER BY last_visit_time DESC
             LIMIT ?2 OFFSET ?3",
        )?;

        let entries = stmt
            .query_map(params![profile_id, limit, offset], |row| {
                Ok(HistoryEntry {
                    id: HistoryId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                    url: row.get(1)?,
                    title: row.get(2)?,
                    visit_count: row.get(3)?,
                    last_visit_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    typed_count: row.get(5)?,
                    profile_id: row.get(6)?,
                    transition_type: match row.get::<_, String>(7)?.as_str() {
                        "Link" => TransitionType::Link,
                        "Typed" => TransitionType::Typed,
                        "AutoBookmark" => TransitionType::AutoBookmark,
                        "AutoSubframe" => TransitionType::AutoSubframe,
                        "ManualSubframe" => TransitionType::ManualSubframe,
                        "Generated" => TransitionType::Generated,
                        "StartPage" => TransitionType::StartPage,
                        "FormSubmit" => TransitionType::FormSubmit,
                        "Reload" => TransitionType::Reload,
                        "Keyword" => TransitionType::Keyword,
                        "KeywordGenerated" => TransitionType::KeywordGenerated,
                        _ => TransitionType::Link,
                    },
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub async fn search_history(
        &self,
        profile_id: &str,
        query: &str,
        limit: usize,
    ) -> CoreResult<Vec<HistoryEntry>> {
        let conn = self.conn.read().await;

        let mut stmt = conn.prepare(
            "SELECT h.id, h.url, h.title, h.visit_count, h.last_visit_time, h.typed_count, h.profile_id, h.transition_type
             FROM history h
             JOIN history_fts fts ON h.rowid = fts.rowid
             WHERE h.profile_id = ?1 AND history_fts MATCH ?2
             ORDER BY h.last_visit_time DESC
             LIMIT ?3",
        )?;

        let entries = stmt
            .query_map(params![profile_id, query, limit], |row| {
                Ok(HistoryEntry {
                    id: HistoryId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                    url: row.get(1)?,
                    title: row.get(2)?,
                    visit_count: row.get(3)?,
                    last_visit_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    typed_count: row.get(5)?,
                    profile_id: row.get(6)?,
                    transition_type: match row.get::<_, String>(7)?.as_str() {
                        "Link" => TransitionType::Link,
                        "Typed" => TransitionType::Typed,
                        "AutoBookmark" => TransitionType::AutoBookmark,
                        "AutoSubframe" => TransitionType::AutoSubframe,
                        "ManualSubframe" => TransitionType::ManualSubframe,
                        "Generated" => TransitionType::Generated,
                        "StartPage" => TransitionType::StartPage,
                        "FormSubmit" => TransitionType::FormSubmit,
                        "Reload" => TransitionType::Reload,
                        "Keyword" => TransitionType::Keyword,
                        "KeywordGenerated" => TransitionType::KeywordGenerated,
                        _ => TransitionType::Link,
                    },
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub async fn delete_history_entry(&self, id: HistoryId) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute("DELETE FROM history WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    pub async fn clear_history(&self, profile_id: &str) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute("DELETE FROM history WHERE profile_id = ?1", params![profile_id])?;
        Ok(())
    }

    pub async fn clear_all_history(&self) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }

    pub async fn get_most_visited(&self, profile_id: &str, limit: usize) -> CoreResult<Vec<HistoryEntry>> {
        let conn = self.conn.read().await;

        let mut stmt = conn.prepare(
            "SELECT id, url, title, visit_count, last_visit_time, typed_count, profile_id, transition_type
             FROM history
             WHERE profile_id = ?1
             ORDER BY visit_count DESC, last_visit_time DESC
             LIMIT ?2",
        )?;

        let entries = stmt
            .query_map(params![profile_id, limit], |row| {
                Ok(HistoryEntry {
                    id: HistoryId(Uuid::parse_str(&row.get::<_, String>(0)?).unwrap()),
                    url: row.get(1)?,
                    title: row.get(2)?,
                    visit_count: row.get(3)?,
                    last_visit_time: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    typed_count: row.get(5)?,
                    profile_id: row.get(6)?,
                    transition_type: match row.get::<_, String>(7)?.as_str() {
                        "Link" => TransitionType::Link,
                        "Typed" => TransitionType::Typed,
                        "AutoBookmark" => TransitionType::AutoBookmark,
                        "AutoSubframe" => TransitionType::AutoSubframe,
                        "ManualSubframe" => TransitionType::ManualSubframe,
                        "Generated" => TransitionType::Generated,
                        "StartPage" => TransitionType::StartPage,
                        "FormSubmit" => TransitionType::FormSubmit,
                        "Reload" => TransitionType::Reload,
                        "Keyword" => TransitionType::Keyword,
                        "KeywordGenerated" => TransitionType::KeywordGenerated,
                        _ => TransitionType::Link,
                    },
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub async fn get_recently_closed(&self, profile_id: &str, limit: usize) -> CoreResult<Vec<HistoryEntry>> {
        // For recently closed tabs, we'd need a separate table
        // For now, return recently visited
        self.get_history(profile_id, limit, 0).await
    }

    pub async fn save(&self) -> CoreResult<()> {
        // SQLite auto-commits, but we can force a checkpoint
        let conn = self.conn.read().await;
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)", [])?;
        Ok(())
    }

    pub fn get_db_path(&self) -> &PathBuf {
        &self.db_path
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self {
            conn: Arc::new(RwLock::new(Connection::open_in_memory().unwrap())),
            db_path: PathBuf::from("."),
        }
    }
}