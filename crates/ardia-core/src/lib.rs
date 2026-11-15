pub mod window;
pub mod tab;
pub mod webview;
pub mod profile;
pub mod settings;
pub mod extensions;
pub mod downloads;
pub mod history;
pub mod bookmarks;
pub mod passwords;
pub mod autocomplete;
pub mod content_blocker;
pub mod sidebar;
pub mod pip;
pub mod telemetry;
pub mod ipc;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    window::WindowManager,
    tab::TabManager,
    profile::ProfileManager,
    settings::SettingsManager,
    extensions::ExtensionManager,
    downloads::DownloadManager,
    history::HistoryManager,
    bookmarks::BookmarkManager,
    passwords::PasswordManager,
    autocomplete::AutocompleteManager,
    content_blocker::ContentBlocker,
    sidebar::SidebarManager,
    pip::PipManager,
    telemetry::TelemetryManager,
};

pub struct ArdiaCore {
    pub window_manager: Arc<RwLock<WindowManager>>,
    pub tab_manager: Arc<RwLock<TabManager>>,
    pub profile_manager: Arc<RwLock<ProfileManager>>,
    pub settings_manager: Arc<RwLock<SettingsManager>>,
    pub extension_manager: Arc<RwLock<ExtensionManager>>,
    pub download_manager: Arc<RwLock<DownloadManager>>,
    pub history_manager: Arc<RwLock<HistoryManager>>,
    pub bookmark_manager: Arc<RwLock<BookmarkManager>>,
    pub password_manager: Arc<RwLock<PasswordManager>>,
    pub autocomplete_manager: Arc<RwLock<AutocompleteManager>>,
    pub content_blocker: Arc<RwLock<ContentBlocker>>,
    pub sidebar_manager: Arc<RwLock<SidebarManager>>,
    pub pip_manager: Arc<RwLock<PipManager>>,
    pub telemetry_manager: Arc<RwLock<TelemetryManager>>,
}

impl ArdiaCore {
    pub async fn new() -> crate::error::CoreResult<Self> {
        info!("Initializing ArdiaCore...");

        let settings_manager = Arc::new(RwLock::new(SettingsManager::new().await?));
        let profile_manager = Arc::new(RwLock::new(ProfileManager::new(settings_manager.clone()).await?));
        let window_manager = Arc::new(RwLock::new(WindowManager::new().await?));
        let tab_manager = Arc::new(RwLock::new(TabManager::new()));
        let extension_manager = Arc::new(RwLock::new(ExtensionManager::new().await?));
        let download_manager = Arc::new(RwLock::new(DownloadManager::new().await?));
        let history_manager = Arc::new(RwLock::new(HistoryManager::new().await?));
        let bookmark_manager = Arc::new(RwLock::new(BookmarkManager::new().await?));
        let password_manager = Arc::new(RwLock::new(PasswordManager::new().await?));
        let autocomplete_manager = Arc::new(RwLock::new(AutocompleteManager::new().await?));
        let content_blocker = Arc::new(RwLock::new(ContentBlocker::new().await?));
        let sidebar_manager = Arc::new(RwLock::new(SidebarManager::new()));
        let pip_manager = Arc::new(RwLock::new(PipManager::new()));
        let telemetry_manager = Arc::new(RwLock::new(TelemetryManager::new()));

        Ok(Self {
            window_manager,
            tab_manager,
            profile_manager,
            settings_manager,
            extension_manager,
            download_manager,
            history_manager,
            bookmark_manager,
            password_manager,
            autocomplete_manager,
            content_blocker,
            sidebar_manager,
            pip_manager,
            telemetry_manager,
        })
    }

    pub async fn shutdown(&self) -> crate::error::CoreResult<()> {
        info!("Shutting down ArdiaCore...");
        // Save all persistent data
        self.settings_manager.write().await.save().await?;
        self.profile_manager.write().await.save().await?;
        self.history_manager.write().await.save().await?;
        self.bookmark_manager.write().await.save().await?;
        self.password_manager.write().await.save().await?;
        Ok(())
    }
}

pub mod error {
    use std::fmt;
    use anyhow::Error as AnyhowError;

    #[derive(Debug)]
    pub enum CoreError {
        Io(std::io::Error),
        Database(rusqlite::Error),
        Config(config::ConfigError),
        WebView2(crate::webview::WebViewError),
        InvalidState(String),
        NotFound(String),
        Serialization(serde_json::Error),
        Encryption(String),
    }

    impl fmt::Display for CoreError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CoreError::Io(e) => write!(f, "IO error: {}", e),
                CoreError::Database(e) => write!(f, "Database error: {}", e),
                CoreError::Config(e) => write!(f, "Config error: {}", e),
                CoreError::WebView2(e) => write!(f, "WebView2 error: {}", e),
                CoreError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
                CoreError::NotFound(msg) => write!(f, "Not found: {}", msg),
                CoreError::Serialization(e) => write!(f, "Serialization error: {}", e),
                CoreError::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            }
        }
    }

    impl std::error::Error for CoreError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                CoreError::Io(e) => Some(e),
                CoreError::Database(e) => Some(e),
                CoreError::Config(e) => Some(e),
                CoreError::WebView2(e) => Some(e),
                CoreError::Serialization(e) => Some(e),
                _ => None,
            }
        }
    }

    impl From<std::io::Error> for CoreError {
        fn from(e: std::io::Error) -> Self {
            CoreError::Io(e)
        }
    }

    impl From<rusqlite::Error> for CoreError {
        fn from(e: rusqlite::Error) -> Self {
            CoreError::Database(e)
        }
    }

    impl From<config::ConfigError> for CoreError {
        fn from(e: config::ConfigError) -> Self {
            CoreError::Config(e)
        }
    }

    impl From<serde_json::Error> for CoreError {
        fn from(e: serde_json::Error) -> Self {
            CoreError::Serialization(e)
        }
    }

    impl From<crate::webview::WebViewError> for CoreError {
        fn from(e: crate::webview::WebViewError) -> Self {
            CoreError::WebView2(e)
        }
    }

    pub type CoreResult<T> = Result<T, CoreError>;
}