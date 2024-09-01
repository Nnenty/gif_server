use axum::response::{Html, IntoResponse};
use reqwest::StatusCode;
use tracing::debug;

use crate::presentation::html::get_home_html;

pub async fn home() -> Result<Html<String>, impl IntoResponse> {
    debug!("Client connected to `/`");

    let home_html = match get_home_html().await {
        Ok(home_html) => home_html,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    Ok(home_html)
}
