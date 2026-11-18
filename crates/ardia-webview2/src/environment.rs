use std::path::Path;
use webview2::{Environment, EnvironmentBuilder};

use crate::{WebViewOptions, WebViewError, WebViewResult};

pub fn create_environment(options: &WebViewOptions) -> WebViewResult<Environment> {
    std::fs::create_dir_all(&options.user_data_folder)?;

    let environment = EnvironmentBuilder::new()
        .with_user_data_folder(&options.user_data_folder)
        .build()?;

    tracing::info!(
        "WebView2 environment created at: {:?}",
        options.user_data_folder
    );

    Ok(environment)
}

pub fn create_environment_with_options(
    user_data_folder: &Path,
    browser_executable_folder: Option<&Path>,
    additional_browser_arguments: Option<&str>,
) -> WebViewResult<Environment> {
    std::fs::create_dir_all(user_data_folder)?;

    let mut builder = EnvironmentBuilder::new()
        .with_user_data_folder(user_data_folder);

    if let Some(folder) = browser_executable_folder {
        builder = builder.with_browser_executable_folder(folder);
    }

    if let Some(args) = additional_browser_arguments {
        builder = builder.with_additional_browser_arguments(args);
    }

    let environment = builder.build()?;

    Ok(environment)
}