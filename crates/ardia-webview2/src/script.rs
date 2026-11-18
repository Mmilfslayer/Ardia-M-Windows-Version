use webview2::{WebView};

use crate::{WebViewError, WebViewResult};

pub fn add_script_to_execute_on_document_created(
    webview: &WebView,
    script: &str,
) -> WebViewResult<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    webview.add_script_to_execute_on_document_created(script, move |result| {
        let _ = tx.send(result);
        Ok(())
    })?;
    rx.recv().unwrap_or(Err(WebViewError::ScriptExecutionFailed(
        "Script execution channel closed".to_string(),
    )))?
}

pub fn remove_script_to_execute_on_document_created(
    webview: &WebView,
    id: &str,
) -> WebViewResult<()> {
    webview.remove_script_to_execute_on_document_created(id)?;
    Ok(())
}

pub fn execute_script(webview: &WebView, script: &str) -> WebViewResult<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    webview.execute_script(script, move |result| {
        let _ = tx.send(result);
        Ok(())
    })?;
    rx.recv().unwrap_or(Err(WebViewError::ScriptExecutionFailed(
        "Script execution channel closed".to_string(),
    )))?
}