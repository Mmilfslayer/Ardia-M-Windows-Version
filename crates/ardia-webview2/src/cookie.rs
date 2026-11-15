use webview2_com::{CoreWebView2, ICoreWebView2, CoreWebView2CookieManager, ICoreWebView2CookieManager};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

pub fn get_cookie_manager(webview: &CoreWebView2) -> WebViewResult<CoreWebView2CookieManager> {
    let manager = webview.get_CookieManager()?;
    Ok(manager)
}

pub async fn get_cookies(
    cookie_manager: &CoreWebView2CookieManager,
    url: &str,
) -> WebViewResult<Vec<webview2_com::CoreWebView2Cookie>> {
    let cookies = cookie_manager.GetCookiesAsync(url).await?;
    let mut result = Vec::new();
    for cookie in cookies {
        result.push(cookie);
    }
    Ok(result)
}

pub async fn get_all_cookies(
    cookie_manager: &CoreWebView2CookieManager,
) -> WebViewResult<Vec<webview2_com::CoreWebView2Cookie>> {
    let cookies = cookie_manager.GetCookiesAsync("").await?;
    let mut result = Vec::new();
    for cookie in cookies {
        result.push(cookie);
    }
    Ok(result)
}

pub fn add_or_update_cookie(
    cookie_manager: &CoreWebView2CookieManager,
    cookie: &webview2_com::CoreWebView2Cookie,
) -> WebViewResult<()> {
    cookie_manager.AddOrUpdateCookie(cookie)?;
    Ok(())
}

pub fn delete_cookie(
    cookie_manager: &CoreWebView2CookieManager,
    cookie: &webview2_com::CoreWebView2Cookie,
) -> WebViewResult<()> {
    cookie_manager.DeleteCookie(cookie)?;
    Ok(())
}

pub fn delete_all_cookies(cookie_manager: &CoreWebView2CookieManager) -> WebViewResult<()> {
    cookie_manager.DeleteAllCookies()?;
    Ok(())
}

pub fn add_cookie_changed_handler<F>(
    cookie_manager: &CoreWebView2CookieManager,
    handler: F,
) -> WebViewResult<i64>
where
    F: Fn(&CoreWebView2CookieManager, &webview2_com::CookieChangedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = cookie_manager.add_CookieChanged(handler)?;
    Ok(token)
}

pub fn remove_cookie_changed_handler(cookie_manager: &CoreWebView2CookieManager, token: i64) -> WebViewResult<()> {
    cookie_manager.remove_CookieChanged(token)?;
    Ok(())
}