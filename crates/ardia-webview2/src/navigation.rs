use webview2::{WebView, NavigationStartingEventArgs, NavigationCompletedEventArgs, EventRegistrationToken, Error as WebView2Error};

use crate::{WebViewError, WebViewResult};

pub fn navigate(webview: &WebView, url: &str) -> WebViewResult<()> {
    webview.navigate(url)?;
    Ok(())
}

pub fn navigate_with_web_resource_request(
    webview: &WebView,
    request: &WebResourceRequest,
) -> WebViewResult<()> {
    webview.navigate_with_web_resource_request(request)?;
    Ok(())
}

pub fn go_back(webview: &WebView) -> WebViewResult<()> {
    if webview.get_can_go_back()? {
        webview.go_back()?;
    }
    Ok(())
}

pub fn go_forward(webview: &WebView) -> WebViewResult<()> {
    if webview.get_can_go_forward()? {
        webview.go_forward()?;
    }
    Ok(())
}

pub fn reload(webview: &WebView) -> WebViewResult<()> {
    webview.reload()?;
    Ok(())
}

pub fn stop(webview: &WebView) -> WebViewResult<()> {
    webview.stop()?;
    Ok(())
}

pub fn can_go_back(webview: &WebView) -> WebViewResult<bool> {
    Ok(webview.get_can_go_back()?)
}

pub fn can_go_forward(webview: &WebView) -> WebViewResult<bool> {
    Ok(webview.get_can_go_forward()?)
}

pub fn add_navigation_starting_handler<F>(webview: &WebView, handler: F) -> WebViewResult<EventRegistrationToken>
where
    F: Fn(WebView, NavigationStartingEventArgs) -> Result<(), WebView2Error> + Send + 'static,
{
    let token = webview.add_navigation_starting(handler)?;
    Ok(token)
}

pub fn add_navigation_completed_handler<F>(webview: &WebView, handler: F) -> WebViewResult<EventRegistrationToken>
where
    F: Fn(WebView, NavigationCompletedEventArgs) -> Result<(), WebView2Error> + Send + 'static,
{
    let token = webview.add_navigation_completed(handler)?;
    Ok(token)
}

pub fn remove_navigation_starting_handler(webview: &WebView, token: EventRegistrationToken) -> WebViewResult<()> {
    webview.remove_navigation_starting(token)?;
    Ok(())
}

pub fn remove_navigation_completed_handler(webview: &WebView, token: EventRegistrationToken) -> WebViewResult<()> {
    webview.remove_navigation_completed(token)?;
    Ok(())
}