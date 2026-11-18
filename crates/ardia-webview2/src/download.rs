use std::path::Path;
use webview2::{WebView, DownloadStartingEventArgs, DownloadOperation, EventRegistrationToken};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

#[derive(Debug, Clone)]
pub struct DownloadInfo {
    pub id: String,
    pub url: String,
    pub suggested_filename: String,
    pub state: DownloadState,
    pub progress: DownloadProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadState {
    InProgress,
    Completed,
    Interrupted,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub bytes_received: u64,
    pub total_bytes: u64,
    pub percent_complete: f64,
}

pub fn get_download_info(args: &DownloadStartingEventArgs) -> WebViewResult<DownloadInfo> {
    let operation = args.get_download_operation()?;
    let id = operation.get_id()?;
    let url = operation.get_uri()?;
    let suggested_filename = operation.get_result_file_path()?;

    Ok(DownloadInfo {
        id,
        url,
        suggested_filename,
        state: DownloadState::InProgress,
        progress: DownloadProgress {
            bytes_received: 0,
            total_bytes: 0,
            percent_complete: 0.0,
        },
    })
}

pub fn set_download_path(args: &DownloadStartingEventArgs, path: &Path) -> WebViewResult<()> {
    let operation = args.get_download_operation()?;
    operation.put_result_file_path(path.to_str().unwrap())?;
    Ok(())
}

pub fn get_download_operation(args: &DownloadStartingEventArgs) -> WebViewResult<DownloadOperation> {
    let operation = args.get_download_operation()?;
    Ok(operation)
}

pub fn add_download_starting_handler<F>(webview: &WebView, handler: F) -> WebViewResult<EventRegistrationToken>
where
    F: Fn(WebView, DownloadStartingEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_download_starting(handler)?;
    Ok(token)
}

pub fn remove_download_starting_handler(webview: &WebView, token: EventRegistrationToken) -> WebViewResult<()> {
    webview.remove_download_starting(token)?;
    Ok(())
}

pub fn add_download_operation_state_changed_handler<F>(
    operation: &DownloadOperation,
    handler: F,
) -> WebViewResult<EventRegistrationToken>
where
    F: Fn(DownloadOperation, webview2::SourceChangedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = operation.add_state_changed(handler)?;
    Ok(token)
}

pub fn cancel_download(operation: &DownloadOperation) -> WebViewResult<()> {
    operation.cancel()?;
    Ok(())
}

pub fn pause_download(operation: &DownloadOperation) -> WebViewResult<()> {
    operation.pause()?;
    Ok(())
}

pub fn resume_download(operation: &DownloadOperation) -> WebViewResult<()> {
    operation.resume()?;
    Ok(())
}

pub fn get_download_progress(operation: &DownloadOperation) -> WebViewResult<DownloadProgress> {
    let bytes_received = operation.get_bytes_received()?;
    let total_bytes = operation.get_total_bytes_to_receive()?;
    let percent_complete = if total_bytes > 0 {
        (bytes_received as f64 / total_bytes as f64) * 100.0
    } else {
        0.0
    };

    Ok(DownloadProgress {
        bytes_received,
        total_bytes,
        percent_complete,
    })
}

pub fn get_download_state(operation: &DownloadOperation) -> WebViewResult<DownloadState> {
    let state = operation.get_state()?;
    Ok(match state {
        0 => DownloadState::InProgress,
        1 => DownloadState::Completed,
        2 => DownloadState::Interrupted,
        3 => DownloadState::Cancelled,
        _ => DownloadState::InProgress,
    })
}