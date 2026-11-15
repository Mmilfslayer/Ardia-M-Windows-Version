use std::sync::Arc;
use webview2_com::{CoreWebView2Controller, CoreWebView2Environment};
use windows::Win32::Foundation::{HWND, RECT};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

pub async fn create_controller(
    environment: &CoreWebView2Environment,
    hwnd: HWND,
) -> WebViewResult<CoreWebView2Controller> {
    // Create the WebView2 controller
    let controller = environment.CreateControllerAsync(hwnd).await?;

    // Set initial bounds to fill the window
    let mut rect = RECT::default();
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::GetClientRect(hwnd, &mut rect)?;
    }
    controller.put_Bounds(rect)?;

    // Enable default context menu
    // Note: This can be customized later

    tracing::info!("WebView2 controller created for HWND: {:?}", hwnd.0);

    Ok(controller)
}

pub fn set_controller_bounds(controller: &CoreWebView2Controller, bounds: RECT) -> WebViewResult<()> {
    controller.put_Bounds(bounds)?;
    Ok(())
}

pub fn get_controller_bounds(controller: &CoreWebView2Controller) -> WebViewResult<RECT> {
    let bounds = controller.get_Bounds()?;
    Ok(bounds)
}

pub fn set_zoom_factor(controller: &CoreWebView2Controller, zoom: f64) -> WebViewResult<()> {
    controller.put_ZoomFactor(zoom)?;
    Ok(())
}

pub fn get_zoom_factor(controller: &CoreWebView2Controller) -> WebViewResult<f64> {
    let zoom = controller.get_ZoomFactor()?;
    Ok(zoom)
}