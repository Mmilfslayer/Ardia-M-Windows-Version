use webview2::{WebView, CookieManager};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

pub fn get_cookie_manager(webview: &WebView) -> WebViewResult<CookieManager> {
    let manager = webview.get_cookie_manager()?;
    Ok(manager)
}

pub async fn get_cookies(
    cookie_manager: &CookieManager,
    url: &str,
) -> WebViewResult<Vec<webview2::Cookie>> {
    let cookies = cookie_manager.get_cookies(url).await?;
    Ok(cookies)
}

pub async fn get_all_cookies(
    cookie_manager: &CookieManager,
) -> WebViewResult<Vec<webview2::Cookie>> {
    let cookies = cookie_manager.get_cookies("").await?;
    Ok(cookies)
}

pub fn add_or_update_cookie(
    cookie_manager: &CookieManager,
    cookie: &webview2::Cookie,
) -> WebViewResult<()> {
    cookie_manager.add_or_update_cookie(cookie)?;
    Ok(())
}

pub fn delete_cookie(
    cookie_manager: &CookieManager,
    cookie: &webview2::Cookie,
) -> WebViewResult<()> {
    cookie_manager.delete_cookie(cookie)?;
    Ok(())
}

pub fn delete_all_cookies(cookie_manager: &CookieManager) -> WebViewResult<()> {
    cookie_manager.delete_all_cookies()?;
    Ok(())
}

pub fn add_cookie_changed_handler<F>(
    cookie_manager: &CookieManager,
    handler: F,
) -> WebViewResult<webview2::EventRegistrationToken>
where
    F: Fn(CookieManager, webview2::SourceChangedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = cookie_manager.add_cookie_changed(handler)?;
    Ok(token)
}

pub fn remove_cookie_changed_handler(cookie_manager: &CookieManager, token: webview2::EventRegistrationToken) -> WebViewResult<()> {
    cookie_manager.remove_cookie_changed(token)?;
    Ok(())
}