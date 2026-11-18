use webview2::{Environment, Controller};
use windows::Win32::Foundation::{HWND, RECT};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

pub async fn create_controller(
    environment: &Environment,
    hwnd: HWND,
) -> WebViewResult<Controller> {
    let controller = environment.create_controller(hwnd).await?;

    let mut rect = RECT::default();
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::GetClientRect(hwnd, &mut rect)?;
    }
    controller.put_bounds(rect)?;

    tracing::info!("WebView2 controller created for HWND: {:?}", hwnd.0);

    Ok(controller)
}

pub fn set_controller_bounds(controller: &Controller, bounds: RECT) -> WebViewResult<()> {
    controller.put_bounds(bounds)?;
    Ok(())
}

pub fn get_controller_bounds(controller: &Controller) -> WebViewResult<RECT> {
    let bounds = controller.get_bounds()?;
    Ok(bounds)
}

pub fn set_zoom_factor(controller: &Controller, zoom: f64) -> WebViewResult<()> {
    controller.put_zoom_factor(zoom)?;
    Ok(())
}

pub fn get_zoom_factor(controller: &Controller) -> WebViewResult<f64> {
    let zoom = controller.get_zoom_factor()?;
    Ok(zoom)
}