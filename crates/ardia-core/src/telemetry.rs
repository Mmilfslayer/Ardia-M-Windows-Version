use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, OptionalExtension};

use crate::error::CoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetryEventType {
    AppStart,
    AppExit,
    WindowOpen,
    WindowClose,
    TabOpen,
    TabClose,
    TabNavigate,
    TabReload,
    Search,
    BookmarkAdd,
    BookmarkRemove,
    HistoryClear,
    DownloadStart,
    DownloadComplete,
    DownloadCancel,
    ExtensionInstall,
    ExtensionEnable,
    ExtensionDisable,
    SettingsChange,
    ThemeChange,
    ProfileSwitch,
    PasswordSave,
    PasswordFill,
    ContentBlock,
    ContentAllow,
    PipEnter,
    PipExit,
    Crash,
    Error,
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub id: String,
    pub event_type: TelemetryEventType,
    pub timestamp: u64,
    pub session_id: String,
    pub profile_id: Option<String>,
    pub properties: HashMap<String, serde_json::Value>,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: u64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub duration_ms: Option<u64>,
    pub window_count: u32,
    pub tab_count: u32,
    pub crashes: u32,
}

pub struct TelemetryManager {
    conn: Arc<RwLock<Connection>>,
    db_path: PathBuf,
    session_id: String,
    enabled: bool,
    event_buffer: Arc<RwLock<Vec<TelemetryEvent>>>,
    flush_interval: Duration,
    last_flush: Instant,
}

impl TelemetryManager {
    pub async fn new() -> CoreResult<Self> {
        let db_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ardia")
            .join("telemetry.db");

        std::fs::create_dir_all(db_path.parent().unwrap())?;

        let conn = Connection::open(&db_path)?;

        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                event_type TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                session_id TEXT NOT NULL,
                profile_id TEXT,
                properties TEXT NOT NULL,
                duration_ms INTEGER
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_session ON events(session_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS performance_metrics (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                value REAL NOT NULL,
                unit TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                tags TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_perf_name ON performance_metrics(name)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_perf_timestamp ON performance_metrics(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                session_id TEXT PRIMARY KEY,
                start_time INTEGER NOT NULL,
                end_time INTEGER,
                duration_ms INTEGER,
                window_count INTEGER NOT NULL DEFAULT 0,
                tab_count INTEGER NOT NULL DEFAULT 0,
                crashes INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        let session_id = Uuid::new_v4().to_string();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Insert session start
        conn.execute(
            "INSERT INTO sessions (session_id, start_time, window_count, tab_count, crashes)
             VALUES (?1, ?2, 0, 0, 0)",
            params![session_id, now],
        )?;

        Ok(Self {
            conn: Arc::new(RwLock::new(conn)),
            db_path,
            session_id,
            enabled: true,
            event_buffer: Arc::new(RwLock::new(Vec::new())),
            flush_interval: Duration::from_secs(30),
            last_flush: Instant::now(),
        })
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub async fn record_event(
        &self,
        event_type: TelemetryEventType,
        profile_id: Option<String>,
        properties: HashMap<String, serde_json::Value>,
        duration_ms: Option<u64>,
    ) -> CoreResult<()> {
        if !self.enabled {
            return Ok(());
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let event = TelemetryEvent {
            id: Uuid::new_v4().to_string(),
            event_type,
            timestamp: now,
            session_id: self.session_id.clone(),
            profile_id,
            properties,
            duration_ms,
        };

        {
            let mut buffer = self.event_buffer.write().await;
            buffer.push(event);

            // Flush if buffer is large enough
            if buffer.len() >= 100 {
                drop(buffer);
                self.flush().await?;
            }
        }

        // Periodic flush
        if self.last_flush.elapsed() >= self.flush_interval {
            self.flush().await?;
        }

        Ok(())
    }

    pub async fn record_performance(
        &self,
        name: &str,
        value: f64,
        unit: &str,
        tags: HashMap<String, String>,
    ) -> CoreResult<()> {
        if !self.enabled {
            return Ok(());
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let metric = PerformanceMetric {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            value,
            unit: unit.to_string(),
            timestamp: now,
            tags,
        };

        let conn = self.conn.read().await;
        conn.execute(
            "INSERT INTO performance_metrics (id, name, value, unit, timestamp, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                metric.id,
                metric.name,
                metric.value,
                metric.unit,
                metric.timestamp,
                serde_json::to_string(&metric.tags)?,
            ],
        )?;

        Ok(())
    }

    pub async fn record_timing(&self, name: &str, duration: Duration, tags: HashMap<String, String>) -> CoreResult<()> {
        self.record_performance(name, duration.as_millis() as f64, "ms", tags).await
    }

    pub async fn increment_counter(&self, name: &str, value: f64, tags: HashMap<String, String>) -> CoreResult<()> {
        self.record_performance(name, value, "count", tags).await
    }

    pub async fn record_gauge(&self, name: &str, value: f64, unit: &str, tags: HashMap<String, String>) -> CoreResult<()> {
        self.record_performance(name, value, unit, tags).await
    }

    pub async fn flush(&self) -> CoreResult<()> {
        let events = {
            let mut buffer = self.event_buffer.write().await;
            std::mem::take(&mut *buffer)
        };

        if events.is_empty() {
            return Ok(());
        }

        let conn = self.conn.read().await;
        let tx = conn.transaction()?;

        for event in events {
            tx.execute(
                "INSERT INTO events (id, event_type, timestamp, session_id, profile_id, properties, duration_ms)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    event.id,
                    format!("{:?}", event.event_type),
                    event.timestamp,
                    event.session_id,
                    event.profile_id,
                    serde_json::to_string(&event.properties)?,
                    event.duration_ms,
                ],
            )?;
        }

        tx.commit()?;
        self.last_flush = Instant::now();

        Ok(())
    }

    pub async fn end_session(&self) -> CoreResult<()> {
        self.flush().await?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let conn = self.conn.read().await;
        conn.execute(
            "UPDATE sessions SET end_time = ?1, duration_ms = ?2 WHERE session_id = ?3",
            params![now, now - self.get_session_start_time().await?, self.session_id],
        )?;

        Ok(())
    }

    async fn get_session_start_time(&self) -> CoreResult<u64> {
        let conn = self.conn.read().await;
        let start_time: u64 = conn.query_row(
            "SELECT start_time FROM sessions WHERE session_id = ?1",
            params![self.session_id],
            |row| row.get(0),
        )?;
        Ok(start_time)
    }

    pub async fn get_session_info(&self) -> CoreResult<Option<SessionInfo>> {
        let conn = self.conn.read().await;
        let session = conn.query_row(
            "SELECT session_id, start_time, end_time, duration_ms, window_count, tab_count, crashes
             FROM sessions WHERE session_id = ?1",
            params![self.session_id],
            |row| {
                Ok(SessionInfo {
                    session_id: row.get(0)?,
                    start_time: row.get(1)?,
                    end_time: row.get(2)?,
                    duration_ms: row.get(3)?,
                    window_count: row.get(4)?,
                    tab_count: row.get(5)?,
                    crashes: row.get(6)?,
                })
            },
        ).optional()?;

        Ok(session)
    }

    pub async fn get_events(
        &self,
        event_type: Option<TelemetryEventType>,
        limit: usize,
    ) -> CoreResult<Vec<TelemetryEvent>> {
        let conn = self.conn.read().await;

        let (query, params_list): (String, Vec<&dyn rusqlite::ToSql>) = if let Some(et) = event_type {
            (
                "SELECT id, event_type, timestamp, session_id, profile_id, properties, duration_ms
                 FROM events
                 WHERE session_id = ?1 AND event_type = ?2
                 ORDER BY timestamp DESC
                 LIMIT ?3".to_string(),
                vec![&self.session_id, &format!("{:?}", et), &(limit as i64)],
            )
        } else {
            (
                "SELECT id, event_type, timestamp, session_id, profile_id, properties, duration_ms
                 FROM events
                 WHERE session_id = ?1
                 ORDER BY timestamp DESC
                 LIMIT ?2".to_string(),
                vec![&self.session_id, &(limit as i64)],
            )
        };

        let mut stmt = conn.prepare(&query)?;
        let events = stmt.query_map(params_list.as_slice(), |row| {
            Ok(TelemetryEvent {
                id: row.get(0)?,
                event_type: match row.get::<_, String>(1)?.as_str() {
                    "AppStart" => TelemetryEventType::AppStart,
                    "AppExit" => TelemetryEventType::AppExit,
                    "WindowOpen" => TelemetryEventType::WindowOpen,
                    "WindowClose" => TelemetryEventType::WindowClose,
                    "TabOpen" => TelemetryEventType::TabOpen,
                    "TabClose" => TelemetryEventType::TabClose,
                    "TabNavigate" => TelemetryEventType::TabNavigate,
                    "TabReload" => TelemetryEventType::TabReload,
                    "Search" => TelemetryEventType::Search,
                    "BookmarkAdd" => TelemetryEventType::BookmarkAdd,
                    "BookmarkRemove" => TelemetryEventType::BookmarkRemove,
                    "HistoryClear" => TelemetryEventType::HistoryClear,
                    "DownloadStart" => TelemetryEventType::DownloadStart,
                    "DownloadComplete" => TelemetryEventType::DownloadComplete,
                    "DownloadCancel" => TelemetryEventType::DownloadCancel,
                    "ExtensionInstall" => TelemetryEventType::ExtensionInstall,
                    "ExtensionEnable" => TelemetryEventType::ExtensionEnable,
                    "ExtensionDisable" => TelemetryEventType::ExtensionDisable,
                    "SettingsChange" => TelemetryEventType::SettingsChange,
                    "ThemeChange" => TelemetryEventType::ThemeChange,
                    "ProfileSwitch" => TelemetryEventType::ProfileSwitch,
                    "PasswordSave" => TelemetryEventType::PasswordSave,
                    "PasswordFill" => TelemetryEventType::PasswordFill,
                    "ContentBlock" => TelemetryEventType::ContentBlock,
                    "ContentAllow" => TelemetryEventType::ContentAllow,
                    "PipEnter" => TelemetryEventType::PipEnter,
                    "PipExit" => TelemetryEventType::PipExit,
                    "Crash" => TelemetryEventType::Crash,
                    "Error" => TelemetryEventType::Error,
                    "Performance" => TelemetryEventType::Performance,
                    _ => TelemetryEventType::Error,
                },
                timestamp: row.get(2)?,
                session_id: row.get(3)?,
                profile_id: row.get(4)?,
                properties: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
                duration_ms: row.get(6)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(events)
    }

    pub async fn get_performance_metrics(
        &self,
        name: Option<&str>,
        limit: usize,
    ) -> CoreResult<Vec<PerformanceMetric>> {
        let conn = self.conn.read().await;

        let (query, params_list): (String, Vec<&dyn rusqlite::ToSql>) = if let Some(n) = name {
            (
                "SELECT id, name, value, unit, timestamp, tags
                 FROM performance_metrics
                 WHERE name = ?1
                 ORDER BY timestamp DESC
                 LIMIT ?2".to_string(),
                vec![n, &(limit as i64)],
            )
        } else {
            (
                "SELECT id, name, value, unit, timestamp, tags
                 FROM performance_metrics
                 ORDER BY timestamp DESC
                 LIMIT ?1".to_string(),
                vec![&(limit as i64)],
            )
        };

        let mut stmt = conn.prepare(&query)?;
        let metrics = stmt.query_map(params_list.as_slice(), |row| {
            Ok(PerformanceMetric {
                id: row.get(0)?,
                name: row.get(1)?,
                value: row.get(2)?,
                unit: row.get(3)?,
                timestamp: row.get(4)?,
                tags: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(metrics)
    }

    pub async fn increment_window_count(&self) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute(
            "UPDATE sessions SET window_count = window_count + 1 WHERE session_id = ?1",
            params![self.session_id],
        )?;
        Ok(())
    }

    pub async fn decrement_window_count(&self) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute(
            "UPDATE sessions SET window_count = window_count - 1 WHERE session_id = ?1",
            params![self.session_id],
        )?;
        Ok(())
    }

    pub async fn increment_tab_count(&self) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute(
            "UPDATE sessions SET tab_count = tab_count + 1 WHERE session_id = ?1",
            params![self.session_id],
        )?;
        Ok(())
    }

    pub async fn decrement_tab_count(&self) -> CoreResult<()> {
        let conn = self.conn.read().await;
        conn.execute(
            "UPDATE sessions SET tab_count = tab_count - 1 WHERE session_id = ?1",
            params![self.session_id],
        )?;
        Ok(())
    }

    pub async fn record_crash(&self, error: &str) -> CoreResult<()> {
        let mut properties = HashMap::new();
        properties.insert("error".to_string(), serde_json::Value::String(error.to_string()));

        self.record_event(TelemetryEventType::Crash, None, properties, None).await?;

        let conn = self.conn.read().await;
        conn.execute(
            "UPDATE sessions SET crashes = crashes + 1 WHERE session_id = ?1",
            params![self.session_id],
        )?;

        Ok(())
    }

    pub async fn record_error(&self, error: &str, context: HashMap<String, serde_json::Value>) -> CoreResult<()> {
        let mut properties = context;
        properties.insert("error".to_string(), serde_json::Value::String(error.to_string()));

        self.record_event(TelemetryEventType::Error, None, properties, None).await
    }

    pub fn get_db_path(&self) -> &PathBuf {
        &self.db_path
    }
}

impl Default for TelemetryManager {
    fn default() -> Self {
        Self {
            conn: Arc::new(RwLock::new(Connection::open_in_memory().unwrap())),
            db_path: PathBuf::from("."),
            session_id: String::new(),
            enabled: false,
            event_buffer: Arc::new(RwLock::new(Vec::new())),
            flush_interval: Duration::from_secs(30),
            last_flush: Instant::now(),
        }
    }
}

// Convenience macros for telemetry
#[macro_export]
macro_rules! telemetry_event {
    ($manager:expr, $event_type:expr) => {
        $manager.record_event($event_type, None, HashMap::new(), None).await.ok();
    };
    ($manager:expr, $event_type:expr, $profile_id:expr) => {
        $manager.record_event($event_type, Some($profile_id), HashMap::new(), None).await.ok();
    };
    ($manager:expr, $event_type:expr, $profile_id:expr, $props:expr) => {
        $manager.record_event($event_type, Some($profile_id), $props, None).await.ok();
    };
}

#[macro_export]
macro_rules! telemetry_timing {
    ($manager:expr, $name:expr, $duration:expr) => {
        $manager.record_timing($name, $duration, HashMap::new()).await.ok();
    };
    ($manager:expr, $name:expr, $duration:expr, $tags:expr) => {
        $manager.record_timing($name, $duration, $tags).await.ok();
    };
}