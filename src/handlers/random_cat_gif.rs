use axum::response::{Html, IntoResponse};
use reqwest::StatusCode;
use tracing::debug;

use crate::{
    presentation::html::get_random_cat_gif_html,
    tenor::queries::random_cat_gif as random_cat_gif_query,
};

pub async fn random_cat_gif() -> Result<Html<String>, impl IntoResponse> {
    debug!("Client connected to `/random_cat_gif`");

    let response = match random_cat_gif_query().await {
        Ok(home_html) => home_html,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    let random_cat_gif_html = match get_random_cat_gif_html(
        response.get_first_gif_url(),
        response.get_first_content_description(),
    )
    .await
    {
        Ok(home_html) => home_html,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    Ok(random_cat_gif_html)
}
