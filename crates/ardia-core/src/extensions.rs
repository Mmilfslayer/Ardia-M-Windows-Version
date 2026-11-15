use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExtensionId(pub Uuid);

impl ExtensionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for ExtensionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub manifest_version: u32,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub icons: HashMap<String, String>,
    pub permissions: Vec<String>,
    pub host_permissions: Vec<String>,
    pub content_scripts: Vec<ContentScript>,
    pub background: Option<BackgroundScript>,
    pub action: Option<ActionConfig>,
    pub sidebar: Option<SidebarConfig>,
    pub options_page: Option<String>,
    pub homepage_url: Option<String>,
    pub minimum_chrome_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentScript {
    pub matches: Vec<String>,
    pub exclude_matches: Vec<String>,
    pub js: Vec<String>,
    pub css: Vec<String>,
    pub run_at: RunAt,
    pub all_frames: bool,
    pub match_about_blank: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunAt {
    DocumentStart,
    DocumentEnd,
    DocumentIdle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundScript {
    pub scripts: Vec<String>,
    pub persistent: bool,
    pub type_: BackgroundType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackgroundType {
    Persistent,
    EventPage,
    ServiceWorker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    pub default_title: Option<String>,
    pub default_icon: HashMap<String, String>,
    pub default_popup: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarConfig {
    pub default_title: String,
    pub default_icon: String,
    pub default_panel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub id: ExtensionId,
    pub manifest: ExtensionManifest,
    pub path: PathBuf,
    pub enabled: bool,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_builtin: bool,
}

pub struct ExtensionManager {
    extensions: HashMap<ExtensionId, Arc<RwLock<Extension>>>,
    extensions_dir: PathBuf,
    content_scripts: HashMap<String, Vec<String>>, // URL pattern -> script IDs
}

impl ExtensionManager {
    pub async fn new() -> CoreResult<Self> {
        let extensions_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ardia")
            .join("Extensions");

        std::fs::create_dir_all(&extensions_dir)?;

        let mut manager = Self {
            extensions: HashMap::new(),
            extensions_dir,
            content_scripts: HashMap::new(),
        };

        // Load installed extensions
        manager.load_extensions().await?;

        Ok(manager)
    }

    async fn load_extensions(&mut self) -> CoreResult<()> {
        let entries = std::fs::read_dir(&self.extensions_dir)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("manifest.json");
                if manifest_path.exists() {
                    if let Ok(extension) = self.load_extension_from_path(&path).await {
                        self.extensions.insert(extension.id, Arc::new(RwLock::new(extension)));
                    }
                }
            }
        }
        Ok(())
    }

    async fn load_extension_from_path(&self, path: &Path) -> CoreResult<Extension> {
        let manifest_path = path.join("manifest.json");
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: ExtensionManifest = serde_json::from_str(&content)?;

        let id = ExtensionId::new();
        let metadata_path = path.join("extension_meta.json");
        let (installed_at, updated_at, is_builtin, enabled) = if metadata_path.exists() {
            let meta_content = std::fs::read_to_string(&metadata_path)?;
            let meta: serde_json::Value = serde_json::from_str(&meta_content)?;
            (
                meta.get("installed_at")
                    .and_then(|v| v.as_str())
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(chrono::Utc::now),
                meta.get("updated_at")
                    .and_then(|v| v.as_str())
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(chrono::Utc::now),
                meta.get("is_builtin").and_then(|v| v.as_bool()).unwrap_or(false),
                meta.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
            )
        } else {
            (chrono::Utc::now(), chrono::Utc::now(), false, true)
        };

        Ok(Extension {
            id,
            manifest,
            path: path.to_path_buf(),
            enabled,
            installed_at,
            updated_at,
            is_builtin,
        })
    }

    pub async fn install_extension(&mut self, source_path: &Path) -> CoreResult<ExtensionId> {
        let manifest_path = source_path.join("manifest.json");
        if !manifest_path.exists() {
            return Err(crate::error::CoreError::InvalidState("No manifest.json found".to_string()));
        }

        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: ExtensionManifest = serde_json::from_str(&content)?;

        // Validate manifest version
        if manifest.manifest_version < 2 || manifest.manifest_version > 3 {
            return Err(crate::error::CoreError::InvalidState("Unsupported manifest version".to_string()));
        }

        // Create extension directory
        let extension_name = manifest.name.replace(' ', "_");
        let extension_dir = self.extensions_dir.join(&extension_name);

        if extension_dir.exists() {
            // Add version suffix
            let versioned_name = format!("{}_{}", extension_name, manifest.version);
            let versioned_dir = self.extensions_dir.join(&versioned_name);
            std::fs::create_dir_all(&versioned_dir)?;
            // Copy files
            self.copy_dir(source_path, &versioned_dir)?;
            return self.load_extension_from_path(&versioned_dir).await.map(|ext| {
                self.extensions.insert(ext.id, Arc::new(RwLock::new(ext.clone())));
                ext.id
            });
        }

        std::fs::create_dir_all(&extension_dir)?;
        self.copy_dir(source_path, &extension_dir)?;

        let extension = self.load_extension_from_path(&extension_dir).await?;
        let id = extension.id;
        self.extensions.insert(id, Arc::new(RwLock::new(extension)));

        Ok(id)
    }

    fn copy_dir(&self, src: &Path, dst: &Path) -> CoreResult<()> {
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                std::fs::create_dir_all(&dst_path)?;
                self.copy_dir(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    pub async fn uninstall_extension(&mut self, id: ExtensionId) -> CoreResult<()> {
        if let Some(extension) = self.extensions.remove(&id) {
            let extension = extension.read().await;
            if extension.path.exists() {
                std::fs::remove_dir_all(&extension.path)?;
            }
        }
        Ok(())
    }

    pub async fn enable_extension(&self, id: ExtensionId) -> CoreResult<()> {
        if let Some(extension) = self.extensions.get(&id) {
            let mut extension = extension.write().await;
            extension.enabled = true;
            extension.updated_at = chrono::Utc::now();
        }
        Ok(())
    }

    pub async fn disable_extension(&self, id: ExtensionId) -> CoreResult<()> {
        if let Some(extension) = self.extensions.get(&id) {
            let mut extension = extension.write().await;
            extension.enabled = false;
            extension.updated_at = chrono::Utc::now();
        }
        Ok(())
    }

    pub async fn get_extension(&self, id: ExtensionId) -> Option<Arc<RwLock<Extension>>> {
        self.extensions.get(&id).cloned()
    }

    pub async fn list_extensions(&self) -> Vec<ExtensionId> {
        self.extensions.keys().copied().collect()
    }

    pub async fn get_enabled_extensions(&self) -> Vec<Arc<RwLock<Extension>>> {
        let mut result = Vec::new();
        for ext in self.extensions.values() {
            let ext = ext.read().await;
            if ext.enabled {
                result.push(ext.clone());
            }
        }
        result
    }

    pub async fn get_content_scripts_for_url(&self, url: &str) -> Vec<String> {
        // Match content scripts against URL
        let mut scripts = Vec::new();
        for ext in self.extensions.values() {
            let ext = ext.read().await;
            if !ext.enabled {
                continue;
            }
            for content_script in &ext.manifest.content_scripts {
                for pattern in &content_script.matches {
                    if self.url_matches_pattern(url, pattern) {
                        for js_file in &content_script.js {
                            scripts.push(ext.path.join(js_file).to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        scripts
    }

    fn url_matches_pattern(&self, url: &str, pattern: &str) -> bool {
        // Simple glob matching for content script patterns
        // In production, use a proper match pattern implementation
        let regex_pattern = pattern
            .replace(".", "\\.")
            .replace("*", ".*")
            .replace("?", ".");
        if let Ok(re) = regex::Regex::new(&format!("^{}$", regex_pattern)) {
            re.is_match(url)
        } else {
            false
        }
    }

    pub async fn inject_content_scripts(&self, webview: &crate::webview::WebViewWrapper, url: &str) -> CoreResult<()> {
        let scripts = self.get_content_scripts_for_url(url).await;
        for script_path in scripts {
            if let Ok(content) = std::fs::read_to_string(&script_path) {
                webview.add_script_to_execute_on_document_created(&content)?;
            }
        }
        Ok(())
    }
}

impl Default for ExtensionManager {
    fn default() -> Self {
        Self {
            extensions: HashMap::new(),
            extensions_dir: PathBuf::from("."),
            content_scripts: HashMap::new(),
        }
    }
}