use std::io;

use axum::response::Html;

use super::constants::HOME_HTML_PATH;

pub async fn get_home_html() -> Result<Html<String>, io::Error> {
    let home_html = match tokio::fs::read_to_string(HOME_HTML_PATH).await {
        Ok(home_html) => home_html,
        Err(err) => return Err(err),
    };

    Ok(Html(home_html))
}
