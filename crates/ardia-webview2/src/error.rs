use std::fmt;
use webview2::Error as WebView2Error;
use windows::core::Error as WindowsError;

#[derive(Debug)]
pub enum WebViewError {
    WebView2(WebView2Error),
    Windows(WindowsError),
    Io(std::io::Error),
    InvalidArgument(String),
    NotInitialized,
    ScriptExecutionFailed(String),
    NavigationFailed(String),
    PermissionDenied(String),
    DownloadFailed(String),
    EnvironmentCreationFailed(String),
    ControllerCreationFailed(String),
}

impl fmt::Display for WebViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebViewError::WebView2(e) => write!(f, "WebView2 error: {}", e),
            WebViewError::Windows(e) => write!(f, "Windows API error: {}", e),
            WebViewError::Io(e) => write!(f, "IO error: {}", e),
            WebViewError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            WebViewError::NotInitialized => write!(f, "WebView not initialized"),
            WebViewError::ScriptExecutionFailed(msg) => write!(f, "Script execution failed: {}", msg),
            WebViewError::NavigationFailed(msg) => write!(f, "Navigation failed: {}", msg),
            WebViewError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            WebViewError::DownloadFailed(msg) => write!(f, "Download failed: {}", msg),
            WebViewError::EnvironmentCreationFailed(msg) => write!(f, "Environment creation failed: {}", msg),
            WebViewError::ControllerCreationFailed(msg) => write!(f, "Controller creation failed: {}", msg),
        }
    }
}

impl std::error::Error for WebViewError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WebViewError::WebView2(e) => Some(e),
            WebViewError::Windows(e) => Some(e),
            WebViewError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<WebView2Error> for WebViewError {
    fn from(e: WebView2Error) -> Self {
        WebViewError::WebView2(e)
    }
}

impl From<WindowsError> for WebViewError {
    fn from(e: WindowsError) -> Self {
        WebViewError::Windows(e)
    }
}

impl From<std::io::Error> for WebViewError {
    fn from(e: std::io::Error) -> Self {
        WebViewError::Io(e)
    }
}

pub type WebViewResult<T> = Result<T, WebViewError>;