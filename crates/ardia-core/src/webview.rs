use std::path::PathBuf;
use std::sync::Arc;
use ardia_webview2::{WebViewInstance, WebViewOptions, WebViewResult};

use crate::error::CoreResult;

pub struct WebViewWrapper {
    inner: Arc<WebViewInstance>,
}

impl WebViewWrapper {
    pub async fn new(hwnd: windows::Win32::Foundation::HWND, options: WebViewOptions) -> CoreResult<Self> {
        let webview = WebViewInstance::new(hwnd, options).await?;
        Ok(Self {
            inner: Arc::new(webview),
        })
    }

    pub fn navigate(&self, url: &str) -> CoreResult<()> {
        self.inner.navigate(url)?;
        Ok(())
    }

    pub fn execute_script(&self, script: &str) -> CoreResult<String> {
        let result = self.inner.execute_script(script)?;
        Ok(result)
    }

    pub fn add_script_to_execute_on_document_created(&self, script: &str) -> CoreResult<String> {
        let id = self.inner.add_script_to_execute_on_document_created(script)?;
        Ok(id)
    }

    pub fn resize(&self, bounds: windows::Win32::Foundation::RECT) -> CoreResult<()> {
        self.inner.resize(bounds)?;
        Ok(())
    }

    pub fn get_inner(&self) -> &WebViewInstance {
        &self.inner
    }
}

#[derive(Clone)]
pub struct WebViewEnvironment {
    // Holds the WebView2 environment for creating new WebViews
    // This is a placeholder - actual implementation would store the environment
}

impl WebViewEnvironment {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WebViewEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export WebViewOptions for convenience
pub use ardia_webview2::WebViewOptions;