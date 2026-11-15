use std::future::Future;
use std::pin::Pin;
use webview2_com::{CoreWebView2, ICoreWebView2};
use windows::core::Result as WinResult;

use crate::{WebViewError, WebViewResult};

pub fn add_script_to_execute_on_document_created(
    webview: &CoreWebView2,
    script: &str,
) -> WebViewResult<String> {
    let id = webview.AddScriptToExecuteOnDocumentCreated(script)?;
    Ok(id)
}

pub fn remove_script_to_execute_on_document_created(
    webview: &CoreWebView2,
    id: &str,
) -> WebViewResult<()> {
    webview.RemoveScriptToExecuteOnDocumentCreated(id)?;
    Ok(())
}

pub async fn execute_script(webview: &CoreWebView2, script: &str) -> WebViewResult<String> {
    // Use the async ExecuteScript method
    let result = webview.ExecuteScriptAsync(script).await?;
    Ok(result)
}

pub fn execute_script_sync(webview: &CoreWebView2, script: &str) -> WebViewResult<String> {
    // For synchronous execution, we use a callback approach
    let (tx, rx) = std::sync::mpsc::channel();
    webview.ExecuteScript(
        script,
        Box::new(move |error_code, result| {
            let _ = tx.send((error_code, result.map(|s| s.to_string())));
        }),
    )?;

    match rx.recv() {
        Ok((0, Some(result))) => Ok(result),
        Ok((0, None)) => Ok(String::new()),
        Ok((error_code, _)) => Err(WebViewError::ScriptExecutionFailed(format!(
            "Script execution failed with error code: {}",
            error_code
        ))),
        Err(_) => Err(WebViewError::ScriptExecutionFailed(
            "Script execution channel closed".to_string(),
        )),
    }
}

pub async fn execute_script_with_result<F, Fut>(webview: &CoreWebView2, script: &str) -> WebViewResult<String>
where
    F: FnOnce(WinResult<String>) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    let (tx, rx) = tokio::sync::oneshot::channel();

    webview.ExecuteScript(
        script,
        Box::new(move |error_code, result| {
            let outcome = if error_code == 0 {
                Ok(result.unwrap_or_default().to_string())
            } else {
                Err(WebViewError::ScriptExecutionFailed(format!(
                    "Script execution failed with error code: {}",
                    error_code
                )))
            };
            let _ = tx.send(outcome);
        }),
    )?;

    rx.await.unwrap_or_else(|_| Err(WebViewError::ScriptExecutionFailed(
        "Script execution channel closed".to_string(),
    )))
}

pub fn add_host_object_to_script(
    webview: &CoreWebView2,
    name: &str,
    object: &windows::core::IInspectable,
) -> WebViewResult<()> {
    webview.AddHostObjectToScript(name, object)?;
    Ok(())
}

pub fn remove_host_object_from_script(webview: &CoreWebView2, name: &str) -> WebViewResult<()> {
    webview.RemoveHostObjectFromScript(name)?;
    Ok(())
}