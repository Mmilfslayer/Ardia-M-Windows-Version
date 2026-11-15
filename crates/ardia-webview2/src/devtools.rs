use webview2_com::{CoreWebView2, ICoreWebView2};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

pub fn open_devtools(webview: &CoreWebView2) -> WebViewResult<()> {
    webview.OpenDevToolsWindow()?;
    Ok(())
}

pub fn close_devtools(webview: &CoreWebView2) -> WebViewResult<()> {
    webview.CloseDevToolsWindow()?;
    Ok(())
}

pub fn is_devtools_open(webview: &CoreWebView2) -> WebViewResult<bool> {
    // Note: There's no direct API to check if DevTools is open
    // We track this state externally
    Ok(false)
}

pub fn add_devtools_protocol_event_receiver<F>(webview: &CoreWebView2, handler: F) -> WebViewResult<i64>
where
    F: Fn(&CoreWebView2, &webview2_com::DevToolsProtocolEventReceivedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_DevToolsProtocolEventReceived(handler)?;
    Ok(token)
}

pub fn remove_devtools_protocol_event_receiver(webview: &CoreWebView2, token: i64) -> WebViewResult<()> {
    webview.remove_DevToolsProtocolEventReceived(token)?;
    Ok(())
}

pub async fn send_devtools_protocol_command(
    webview: &CoreWebView2,
    method: &str,
    params: Option<&str>,
) -> WebViewResult<String> {
    let result = webview.CallDevToolsProtocolMethodAsync(method, params.unwrap_or("{}")).await?;
    Ok(result)
}

pub fn post_web_message(webview: &CoreWebView2, message: &str) -> WebViewResult<()> {
    webview.PostWebMessageAsJson(message)?;
    Ok(())
}

pub fn post_web_message_as_string(webview: &CoreWebView2, message: &str) -> WebViewResult<()> {
    webview.PostWebMessageAsString(message)?;
    Ok(())
}

pub fn add_web_message_received_handler<F>(webview: &CoreWebView2, handler: F) -> WebViewResult<i64>
where
    F: Fn(&CoreWebView2, &webview2_com::WebMessageReceivedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_WebMessageReceived(handler)?;
    Ok(token)
}

pub fn remove_web_message_received_handler(webview: &CoreWebView2, token: i64) -> WebViewResult<()> {
    webview.remove_WebMessageReceived(token)?;
    Ok(())
}