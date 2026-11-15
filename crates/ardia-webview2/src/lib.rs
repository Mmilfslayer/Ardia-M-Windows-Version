use std::path::PathBuf;
use std::sync::Arc;
use webview2_com::{
    CoreWebView2, CoreWebView2Controller, CoreWebView2Environment,
    CoreWebView2Settings, ICoreWebView2Controller, ICoreWebView2,
    ICoreWebView2Environment, ICoreWebView2Settings,
};
use windows::core::{Interface, Result as WinResult};
use windows::Win32::Foundation::HWND;

pub mod controller;
pub mod environment;
pub mod navigation;
pub mod script;
pub mod cookie;
pub mod permission;
pub mod download;
pub mod devtools;
pub mod error;

pub use error::{WebViewError, WebViewResult};

#[derive(Debug, Clone)]
pub struct WebViewOptions {
    pub user_data_folder: PathBuf,
    pub enable_gpu: bool,
    pub enable_webgl: bool,
    pub background_color: Option<windows::Win32::Graphics::Dwm::COLORREF>,
}

impl Default for WebViewOptions {
    fn default() -> Self {
        Self {
            user_data_folder: dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Ardia"),
            enable_gpu: true,
            enable_webgl: true,
            background_color: None,
        }
    }
}

pub struct WebView {
    pub controller: CoreWebView2Controller,
    pub webview: CoreWebView2,
    pub environment: Arc<CoreWebView2Environment>,
    hwnd: HWND,
}

impl WebView {
    pub async fn new(hwnd: HWND, options: WebViewOptions) -> WebViewResult<Self> {
        let environment = environment::create_environment(&options).await?;
        let controller = controller::create_controller(&environment, hwnd).await?;
        let webview = controller.get_CoreWebView2()?;

        Self::configure_settings(&webview, &options)?;
        Self::setup_event_handlers(&webview)?;

        Ok(Self {
            controller,
            webview,
            environment: Arc::new(environment),
            hwnd,
        })
    }

    fn configure_settings(webview: &CoreWebView2, options: &WebViewOptions) -> WebViewResult<()> {
        let settings: ICoreWebView2Settings = webview.get_Settings()?;

        settings.put_IsScriptEnabled(true)?;
        settings.put_AreDefaultScriptDialogsEnabled(true)?;
        settings.put_IsWebMessageEnabled(true)?;
        settings.put_IsStatusBarEnabled(false)?;
        settings.put_AreDevToolsEnabled(true)?;
        settings.put_IsZoomControlEnabled(true)?;
        settings.put_IsBuiltInErrorPageEnabled(true)?;

        if !options.enable_gpu {
            settings.put_IsHardwareAccelerationEnabled(false)?;
        }

        if !options.enable_webgl {
            // WebGL is controlled via feature flags in newer versions
        }

        if let Some(color) = options.background_color {
            // For transparency effects
            settings.put_DefaultBackgroundColor(color)?;
        }

        Ok(())
    }

    fn setup_event_handlers(webview: &CoreWebView2) -> WebViewResult<()> {
        // Navigation events
        let _nav_token = webview.add_NavigationStarting({
            let webview = webview.clone();
            move |sender, args| {
                tracing::debug!("Navigation starting");
                Ok(())
            }
        })?;

        let _nav_completed_token = webview.add_NavigationCompleted({
            move |sender, args| {
                tracing::debug!("Navigation completed");
                Ok(())
            }
        })?;

        // Permission requests
        let _perm_token = webview.add_PermissionRequested({
            move |sender, args| {
                tracing::debug!("Permission requested");
                Ok(())
            }
        })?;

        // New window requests
        let _new_window_token = webview.add_NewWindowRequested({
            move |sender, args| {
                tracing::debug!("New window requested");
                Ok(())
            }
        })?;

        // Web resource requests (for content blocking)
        let _resource_token = webview.add_WebResourceRequested({
            move |sender, args| {
                tracing::trace!("Web resource requested");
                Ok(())
            }
        })?;

        // Script dialogs
        let _dialog_token = webview.add_ScriptDialogOpening({
            move |sender, args| {
                tracing::debug!("Script dialog opening");
                Ok(())
            }
        })?;

        // Downloads
        let _download_token = webview.add_DownloadStarting({
            move |sender, args| {
                tracing::debug!("Download starting");
                Ok(())
            }
        })?;

        // Context menu
        let _context_menu_token = webview.add_ContextMenuRequested({
            move |sender, args| {
                tracing::debug!("Context menu requested");
                Ok(())
            }
        })?;

        Ok(())
    }

    pub fn navigate(&self, url: &str) -> WebViewResult<()> {
        self.webview.Navigate(url)?;
        Ok(())
    }

    pub async fn execute_script(&self, script: &str) -> WebViewResult<String> {
        script::execute_script(&self.webview, script).await
    }

    pub fn add_script_to_execute_on_document_created(&self, script: &str) -> WebViewResult<String> {
        script::add_script_to_execute_on_document_created(&self.webview, script)
    }

    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn get_controller(&self) -> &CoreWebView2Controller {
        &self.controller
    }

    pub fn get_webview(&self) -> &CoreWebView2 {
        &self.webview
    }

    pub fn resize(&self, bounds: windows::Win32::Foundation::RECT) -> WebViewResult<()> {
        self.controller.put_Bounds(bounds)?;
        Ok(())
    }
}

impl Drop for WebView {
    fn drop(&mut self) {
        let _ = self.controller.Close();
    }
}