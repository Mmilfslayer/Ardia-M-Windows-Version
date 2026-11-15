use webview2_com::{CoreWebView2, ICoreWebView2, PermissionRequestedEventArgs, PermissionKind, PermissionState};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionType {
    Microphone,
    Camera,
    Geolocation,
    Notifications,
    ClipboardRead,
    ClipboardWrite,
    FileReadWrite,
    MultipleAutomaticDownloads,
    PointerLock,
    MidiSysex,
}

impl From<PermissionKind> for PermissionType {
    fn from(kind: PermissionKind) -> Self {
        match kind {
            PermissionKind::Microphone => PermissionType::Microphone,
            PermissionKind::Camera => PermissionType::Camera,
            PermissionKind::Geolocation => PermissionType::Geolocation,
            PermissionKind::Notifications => PermissionType::Notifications,
            PermissionKind::ClipboardRead => PermissionType::ClipboardRead,
            PermissionKind::ClipboardWrite => PermissionType::ClipboardWrite,
            PermissionKind::FileReadWrite => PermissionType::FileReadWrite,
            PermissionKind::MultipleAutomaticDownloads => PermissionType::MultipleAutomaticDownloads,
            PermissionKind::PointerLock => PermissionType::PointerLock,
            PermissionKind::MidiSysex => PermissionType::MidiSysex,
            _ => PermissionType::Microphone, // Default fallback
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionDecision {
    Allow,
    Deny,
    Default,
}

impl From<PermissionDecision> for PermissionState {
    fn from(decision: PermissionDecision) -> Self {
        match decision {
            PermissionDecision::Allow => PermissionState::Allow,
            PermissionDecision::Deny => PermissionState::Deny,
            PermissionDecision::Default => PermissionState::Default,
        }
    }
}

pub fn get_permission_request_info(args: &PermissionRequestedEventArgs) -> WebViewResult<(String, PermissionType, bool)> {
    let uri = args.get_Uri()?.to_string();
    let kind: PermissionType = args.get_PermissionKind()?.into();
    let is_user_initiated = args.get_IsUserInitiated()?.as_bool();
    Ok((uri, kind, is_user_initiated))
}

pub fn set_permission_state(args: &PermissionRequestedEventArgs, decision: PermissionDecision) -> WebViewResult<()> {
    args.put_State(decision.into())?;
    Ok(())
}

pub fn add_permission_requested_handler<F>(webview: &CoreWebView2, handler: F) -> WebViewResult<i64>
where
    F: Fn(&CoreWebView2, &PermissionRequestedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_PermissionRequested(handler)?;
    Ok(token)
}

pub fn remove_permission_requested_handler(webview: &CoreWebView2, token: i64) -> WebViewResult<()> {
    webview.remove_PermissionRequested(token)?;
    Ok(())
}

pub async fn get_permission_state(
    webview: &CoreWebView2,
    url: &str,
    permission_type: PermissionType,
) -> WebViewResult<PermissionState> {
    // Note: This would require calling get_PermissionState on the webview
    // which is available in newer WebView2 versions
    Ok(PermissionState::Default)
}