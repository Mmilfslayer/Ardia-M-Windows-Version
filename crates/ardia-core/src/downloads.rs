use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DownloadId(pub Uuid);

impl DownloadId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for DownloadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {
    pub id: DownloadId,
    pub url: String,
    pub suggested_filename: String,
    pub file_path: Option<PathBuf>,
    pub state: DownloadState,
    pub progress: DownloadProgress,
    pub mime_type: Option<String>,
    pub total_bytes: u64,
    pub received_bytes: u64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub error: Option<String>,
    pub referrer: Option<String>,
    pub tab_id: Option<String>,
    pub can_resume: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadState {
    Pending,
    InProgress,
    Paused,
    Completed,
    Cancelled,
    Interrupted,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub bytes_received: u64,
    pub total_bytes: u64,
    pub percent_complete: f64,
    pub speed_bps: u64, // bytes per second
    pub estimated_time_remaining: Option<u64>, // seconds
}

impl Default for DownloadProgress {
    fn default() -> Self {
        Self {
            bytes_received: 0,
            total_bytes: 0,
            percent_complete: 0.0,
            speed_bps: 0,
            estimated_time_remaining: None,
        }
    }
}

pub struct DownloadManager {
    downloads: HashMap<DownloadId, Arc<RwLock<DownloadItem>>>,
    download_dir: PathBuf,
    active_downloads: usize,
    max_concurrent_downloads: usize,
}

impl DownloadManager {
    pub async fn new() -> CoreResult<Self> {
        let download_dir = dirs::download_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ardia");

        std::fs::create_dir_all(&download_dir)?;

        Ok(Self {
            downloads: HashMap::new(),
            download_dir,
            active_downloads: 0,
            max_concurrent_downloads: 3,
        })
    }

    pub fn set_download_dir(&mut self, path: PathBuf) {
        self.download_dir = path;
        std::fs::create_dir_all(&self.download_dir).ok();
    }

    pub fn get_download_dir(&self) -> &PathBuf {
        &self.download_dir
    }

    pub async fn create_download(
        &mut self,
        url: String,
        suggested_filename: String,
        mime_type: Option<String>,
        referrer: Option<String>,
        tab_id: Option<String>,
    ) -> DownloadId {
        let id = DownloadId::new();
        let download = DownloadItem {
            id,
            url,
            suggested_filename,
            file_path: None,
            state: DownloadState::Pending,
            progress: DownloadProgress::default(),
            mime_type,
            total_bytes: 0,
            received_bytes: 0,
            start_time: chrono::Utc::now(),
            end_time: None,
            error: None,
            referrer,
            tab_id,
            can_resume: false,
        };

        self.downloads.insert(id, Arc::new(RwLock::new(download)));
        id
    }

    pub async fn update_download_path(&self, id: DownloadId, path: PathBuf) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let mut download = download.write().await;
            download.file_path = Some(path);
            download.state = DownloadState::InProgress;
        }
        Ok(())
    }

    pub async fn update_download_progress(
        &self,
        id: DownloadId,
        received: u64,
        total: u64,
    ) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let mut download = download.write().await;
            download.received_bytes = received;
            download.total_bytes = total;
            download.progress.bytes_received = received;
            download.progress.total_bytes = total;
            download.progress.percent_complete = if total > 0 {
                (received as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            download.state = DownloadState::InProgress;
        }
        Ok(())
    }

    pub async fn complete_download(&self, id: DownloadId, file_path: PathBuf) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let mut download = download.write().await;
            download.file_path = Some(file_path);
            download.state = DownloadState::Completed;
            download.progress.percent_complete = 100.0;
            download.end_time = Some(chrono::Utc::now());
        }
        Ok(())
    }

    pub async fn fail_download(&self, id: DownloadId, error: String) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let mut download = download.write().await;
            download.state = DownloadState::Failed;
            download.error = Some(error);
            download.end_time = Some(chrono::Utc::now());
        }
        Ok(())
    }

    pub async fn cancel_download(&self, id: DownloadId) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let mut download = download.write().await;
            download.state = DownloadState::Cancelled;
            download.end_time = Some(chrono::Utc::now());
        }
        Ok(())
    }

    pub async fn pause_download(&self, id: DownloadId) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let mut download = download.write().await;
            if download.state == DownloadState::InProgress {
                download.state = DownloadState::Paused;
            }
        }
        Ok(())
    }

    pub async fn resume_download(&self, id: DownloadId) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let mut download = download.write().await;
            if download.state == DownloadState::Paused || download.state == DownloadState::Interrupted {
                download.state = DownloadState::InProgress;
            }
        }
        Ok(())
    }

    pub async fn get_download(&self, id: DownloadId) -> Option<Arc<RwLock<DownloadItem>>> {
        self.downloads.get(&id).cloned()
    }

    pub async fn list_downloads(&self) -> Vec<DownloadId> {
        self.downloads.keys().copied().collect()
    }

    pub async fn get_active_downloads(&self) -> Vec<Arc<RwLock<DownloadItem>>> {
        let mut result = Vec::new();
        for download in self.downloads.values() {
            let download = download.read().await;
            if matches!(
                download.state,
                DownloadState::InProgress | DownloadState::Pending
            ) {
                result.push(download.clone());
            }
        }
        result
    }

    pub async fn get_recent_downloads(&self, limit: usize) -> Vec<Arc<RwLock<DownloadItem>>> {
        let mut downloads: Vec<_> = self.downloads.values().cloned().collect();
        downloads.sort_by(|a, b| {
            let a_time = a.blocking_read().start_time;
            let b_time = b.blocking_read().start_time;
            b_time.cmp(&a_time)
        });
        downloads.into_iter().take(limit).collect()
    }

    pub async fn clear_completed_downloads(&self) -> CoreResult<()> {
        self.downloads.retain(|_, v| {
            let state = v.blocking_read().state;
            !matches!(state, DownloadState::Completed | DownloadState::Cancelled)
        });
        Ok(())
    }

    pub async fn open_download(&self, id: DownloadId) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let download = download.read().await;
            if let Some(path) = &download.file_path {
                if path.exists() {
                    open::that(path)?;
                }
            }
        }
        Ok(())
    }

    pub async fn show_in_folder(&self, id: DownloadId) -> CoreResult<()> {
        if let Some(download) = self.downloads.get(&id) {
            let download = download.read().await;
            if let Some(path) = &download.file_path {
                if path.exists() {
                    // Use Windows explorer to show file in folder
                    std::process::Command::new("explorer")
                        .args(["/select,", &path.to_string_lossy()])
                        .spawn()?;
                }
            }
        }
        Ok(())
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self {
            downloads: HashMap::new(),
            download_dir: PathBuf::from("."),
            active_downloads: 0,
            max_concurrent_downloads: 3,
        }
    }
}