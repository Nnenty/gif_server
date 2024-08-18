use axum::response::Html;
use reqwest::StatusCode;

use super::constants::HOME_HTML_PATH;

pub async fn get_home_html() -> Result<Html<String>, (StatusCode, String)> {
    let home_html = match tokio::fs::read_to_string(HOME_HTML_PATH).await {
        Ok(home_html) => home_html,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    Ok(Html(home_html))
}
