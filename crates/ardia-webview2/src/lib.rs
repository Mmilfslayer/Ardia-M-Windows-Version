use std::path::PathBuf;
use std::sync::Arc;
use webview2::{
    Environment, EnvironmentBuilder, Controller, WebView, Settings,
    WebResourceRequest, WebResourceResponse, PermissionRequestedEventArgs,
    NavigationStartingEventArgs, NavigationCompletedEventArgs,
    ScriptDialogOpeningEventArgs, NewWindowRequestedEventArgs,
    WebMessageReceivedEventArgs, DownloadStartingEventArgs,
    WebErrorStatus, WebResourceContext,
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
    pub background_color: Option<u32>,
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

pub struct WebViewInstance {
    pub controller: Controller,
    pub webview: WebView,
    pub environment: Arc<Environment>,
    hwnd: HWND,
}

impl WebViewInstance {
    pub async fn new(hwnd: HWND, options: WebViewOptions) -> WebViewResult<Self> {
        let environment = environment::create_environment(&options).await?;
        let controller = controller::create_controller(&environment, hwnd).await?;
        let webview = controller.get_webview()?;

        Self::configure_settings(&webview, &options)?;
        Self::setup_event_handlers(&webview)?;

        Ok(Self {
            controller,
            webview,
            environment: Arc::new(environment),
            hwnd,
        })
    }

    fn configure_settings(webview: &WebView, options: &WebViewOptions) -> WebViewResult<()> {
        let settings: Settings = webview.get_settings()?;

        settings.put_is_script_enabled(true)?;
        settings.put_are_default_script_dialogs_enabled(true)?;
        settings.put_is_web_message_enabled(true)?;
        settings.put_is_status_bar_enabled(false)?;
        settings.put_are_dev_tools_enabled(true)?;
        settings.put_is_zoom_control_enabled(true)?;
        settings.put_is_built_in_error_page_enabled(true)?;

        if !options.enable_gpu {
            settings.put_is_hardware_acceleration_enabled(false)?;
        }

        if !options.enable_webgl {
        }

        if let Some(color) = options.background_color {
            settings.put_default_background_color(color)?;
        }

        Ok(())
    }

    fn setup_event_handlers(webview: &WebView) -> WebViewResult<()> {
        let _nav_token = webview.add_navigation_starting({
            let webview = webview.clone();
            move |sender, args| {
                tracing::debug!("Navigation starting");
                Ok(())
            }
        })?;

        let _nav_completed_token = webview.add_navigation_completed({
            move |sender, args| {
                tracing::debug!("Navigation completed");
                Ok(())
            }
        })?;

        let _perm_token = webview.add_permission_requested({
            move |sender, args| {
                tracing::debug!("Permission requested");
                Ok(())
            }
        })?;

        let _new_window_token = webview.add_new_window_requested({
            move |sender, args| {
                tracing::debug!("New window requested");
                Ok(())
            }
        })?;

        let _resource_token = webview.add_web_resource_requested({
            move |sender, args| {
                tracing::trace!("Web resource requested");
                Ok(())
            }
        })?;

        let _dialog_token = webview.add_script_dialog_opening({
            move |sender, args| {
                tracing::debug!("Script dialog opening");
                Ok(())
            }
        })?;

        let _download_token = webview.add_download_starting({
            move |sender, args| {
                tracing::debug!("Download starting");
                Ok(())
            }
        })?;

        let _context_menu_token = webview.add_contains_full_screen_element_changed({
            move |sender, args| {
                tracing::debug!("Context menu requested");
                Ok(())
            }
        })?;

        Ok(())
    }

    pub fn navigate(&self, url: &str) -> WebViewResult<()> {
        self.webview.navigate(url)?;
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

    pub fn get_controller(&self) -> &Controller {
        &self.controller
    }

    pub fn get_webview(&self) -> &WebView {
        &self.webview
    }

    pub fn resize(&self, bounds: windows::Win32::Foundation::RECT) -> WebViewResult<()> {
        self.controller.put_bounds(bounds)?;
        Ok(())
    }
}

impl Drop for WebViewInstance {
    fn drop(&mut self) {
        let _ = self.controller.close();
    }
}