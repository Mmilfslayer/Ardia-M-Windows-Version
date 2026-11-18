use webview2::{WebView, EventRegistrationToken};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

pub fn open_devtools(webview: &WebView) -> WebViewResult<()> {
    webview.open_dev_tools_window()?;
    Ok(())
}

pub fn close_devtools(webview: &WebView) -> WebViewResult<()> {
    webview.close_dev_tools_window()?;
    Ok(())
}

pub fn is_devtools_open(webview: &WebView) -> WebViewResult<bool> {
    Ok(webview.get_contains_full_screen_element()?)
}

pub fn add_devtools_protocol_event_receiver<F>(webview: &WebView, handler: F) -> WebViewResult<EventRegistrationToken>
where
    F: Fn(WebView, webview2::DevToolsProtocolEventReceivedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_dev_tools_protocol_event_received(handler)?;
    Ok(token)
}

pub fn remove_devtools_protocol_event_receiver(webview: &WebView, token: EventRegistrationToken) -> WebViewResult<()> {
    webview.remove_dev_tools_protocol_event_received(token)?;
    Ok(())
}

pub async fn send_devtools_protocol_command(
    webview: &WebView,
    method: &str,
    params: Option<&str>,
) -> WebViewResult<String> {
    let result = webview.call_dev_tools_protocol_method(method, params.unwrap_or("{}")).await?;
    Ok(result)
}

pub fn post_web_message(webview: &WebView, message: &str) -> WebViewResult<()> {
    webview.post_web_message_as_json(message)?;
    Ok(())
}

pub fn post_web_message_as_string(webview: &WebView, message: &str) -> WebViewResult<()> {
    webview.post_web_message_as_string(message)?;
    Ok(())
}

pub fn add_web_message_received_handler<F>(webview: &WebView, handler: F) -> WebViewResult<EventRegistrationToken>
where
    F: Fn(WebView, webview2::WebMessageReceivedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_web_message_received(handler)?;
    Ok(token)
}

pub fn remove_web_message_received_handler(webview: &WebView, token: EventRegistrationToken) -> WebViewResult<()> {
    webview.remove_web_message_received(token)?;
    Ok(())
}