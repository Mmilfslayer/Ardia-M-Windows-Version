use std::path::Path;
use std::sync::Arc;
use webview2_com::CoreWebView2Environment;
use windows::core::Result as WinResult;

use crate::{WebViewOptions, WebViewError, WebViewResult};

pub async fn create_environment(options: &WebViewOptions) -> WebViewResult<CoreWebView2Environment> {
    // Ensure the user data folder exists
    std::fs::create_dir_all(&options.user_data_folder)?;

    // Create the WebView2 environment with the specified user data folder
    let env_options = CoreWebView2Environment::GetOptions(
        options.user_data_folder.to_str().unwrap(),
        None, // browser_executable_folder - use system default
    );

    let environment = CoreWebView2Environment::CreateAsync(env_options).await?;

    tracing::info!(
        "WebView2 environment created at: {:?}",
        options.user_data_folder
    );

    Ok(environment)
}

pub async fn create_environment_with_options(
    user_data_folder: &Path,
    browser_executable_folder: Option<&Path>,
    additional_browser_arguments: Option<&str>,
) -> WebViewResult<CoreWebView2Environment> {
    std::fs::create_dir_all(user_data_folder)?;

    let env_options = CoreWebView2Environment::GetOptions(
        user_data_folder.to_str().unwrap(),
        browser_executable_folder.map(|p| p.to_str().unwrap()),
    );

    if let Some(args) = additional_browser_arguments {
        // Note: Additional browser arguments are set via environment options in newer WebView2 versions
    }

    let environment = CoreWebView2Environment::CreateAsync(env_options).await?;

    Ok(environment)
}