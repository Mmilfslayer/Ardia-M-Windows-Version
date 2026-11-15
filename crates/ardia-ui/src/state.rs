use std::sync::Arc;
use tokio::sync::RwLock;

use ardia_core::{ArdiaCore, ipc::IpcManager};

pub struct AppState {
    pub core: Arc<RwLock<ArdiaCore>>,
    pub ipc: Arc<IpcManager>,
}

impl AppState {
    pub fn new(core: ArdiaCore, ipc: Arc<IpcManager>) -> Self {
        Self {
            core: Arc::new(RwLock::new(core)),
            ipc,
        }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            core: self.core.clone(),
            ipc: self.ipc.clone(),
        }
    }
}