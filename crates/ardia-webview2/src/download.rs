use std::path::Path;
use webview2_com::{CoreWebView2, ICoreWebView2, DownloadStartingEventArgs, CoreWebView2DownloadOperation};
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
    let operation = args.get_DownloadOperation()?;
    let id = operation.get_Id()?.to_string();
    let url = operation.get_Uri()?.to_string();
    let suggested_filename = operation.get_ResultFilePath()?.to_string();

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
    let operation = args.get_DownloadOperation()?;
    operation.put_ResultFilePath(path.to_str().unwrap())?;
    Ok(())
}

pub fn get_download_operation(args: &DownloadStartingEventArgs) -> WebViewResult<CoreWebView2DownloadOperation> {
    let operation = args.get_DownloadOperation()?;
    Ok(operation)
}

pub fn add_download_starting_handler<F>(webview: &CoreWebView2, handler: F) -> WebViewResult<i64>
where
    F: Fn(&CoreWebView2, &DownloadStartingEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = webview.add_DownloadStarting(handler)?;
    Ok(token)
}

pub fn remove_download_starting_handler(webview: &CoreWebView2, token: i64) -> WebViewResult<()> {
    webview.remove_DownloadStarting(token)?;
    Ok(())
}

pub fn add_download_operation_state_changed_handler<F>(
    operation: &CoreWebView2DownloadOperation,
    handler: F,
) -> WebViewResult<i64>
where
    F: Fn(&CoreWebView2DownloadOperation, &webview2_com::DownloadOperationStateChangedEventArgs) -> WinResult<()> + Send + 'static,
{
    let token = operation.add_StateChanged(handler)?;
    Ok(token)
}

pub fn cancel_download(operation: &CoreWebView2DownloadOperation) -> WebViewResult<()> {
    operation.Cancel()?;
    Ok(())
}

pub fn pause_download(operation: &CoreWebView2DownloadOperation) -> WebViewResult<()> {
    operation.Pause()?;
    Ok(())
}

pub fn resume_download(operation: &CoreWebView2DownloadOperation) -> WebViewResult<()> {
    operation.Resume()?;
    Ok(())
}

pub fn get_download_progress(operation: &CoreWebView2DownloadOperation) -> WebViewResult<DownloadProgress> {
    let bytes_received = operation.get_BytesReceived()?;
    let total_bytes = operation.get_TotalBytesToReceive()?;
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

pub fn get_download_state(operation: &CoreWebView2DownloadOperation) -> WebViewResult<DownloadState> {
    let state = operation.get_State()?;
    Ok(match state {
        webview2_com::DownloadState::InProgress => DownloadState::InProgress,
        webview2_com::DownloadState::Completed => DownloadState::Completed,
        webview2_com::DownloadState::Interrupted => DownloadState::Interrupted,
        webview2_com::DownloadState::Cancelled => DownloadState::Cancelled,
        _ => DownloadState::InProgress,
    })
}