use webview2_com::{CoreWebView2, ICoreWebView2};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

pub fn navigate(webview: &CoreWebView2, url: &str) -> WebViewResult<()> {
    webview.Navigate(url)?;
    Ok(())
}

pub fn navigate_with_web_resource_request(
    webview: &CoreWebView2,
    request: &webview2_com::WebResourceRequest,
) -> WebViewResult<()> {
    webview.NavigateWithWebResourceRequest(request)?;
    Ok(())
}

pub fn go_back(webview: &CoreWebView2) -> WebViewResult<()> {
    if webview.get_CanGoBack()? {
        webview.GoBack()?;
    }
    Ok(())
}

pub fn go_forward(webview: &CoreWebView2) -> WebViewResult<()> {
    if webview.get_CanGoForward()? {
        webview.GoForward()?;
    }
    Ok(())
}

pub fn reload(webview: &CoreWebView2) -> WebViewResult<()> {
    webview.Reload()?;
    Ok(())
}

pub fn stop(webview: &CoreWebView2) -> WebViewResult<()> {
    webview.Stop()?;
    Ok(())
}

pub fn can_go_back(webview: &CoreWebView2) -> WebViewResult<bool> {
    Ok(webview.get_CanGoBack()?)
}

pub fn can_go_forward(webview: &CoreWebView2) -> WebViewResult<bool> {
    Ok(webview.get_CanGoForward()?)
}

pub fn add_navigation_starting_handler<F>(webview: &CoreWebView2, handler: F) -> WebViewResult<i64>
where
    F: Fn(&CoreWebView2, &webview2_com::NavigationStartingEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_NavigationStarting(handler)?;
    Ok(token)
}

pub fn add_navigation_completed_handler<F>(webview: &CoreWebView2, handler: F) -> WebViewResult<i64>
where
    F: Fn(&CoreWebView2, &webview2_com::NavigationCompletedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_NavigationCompleted(handler)?;
    Ok(token)
}

pub fn remove_navigation_starting_handler(webview: &CoreWebView2, token: i64) -> WebViewResult<()> {
    webview.remove_NavigationStarting(token)?;
    Ok(())
}

pub fn remove_navigation_completed_handler(webview: &CoreWebView2, token: i64) -> WebViewResult<()> {
    webview.remove_NavigationCompleted(token)?;
    Ok(())
}