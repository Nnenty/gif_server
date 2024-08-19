use axum::response::Html;
use reqwest::StatusCode;

use crate::tenor::TenorResults;

use super::tera::RandomCatGifTera;

pub async fn get_random_cat_gif_html(
    tenor_results: TenorResults,
) -> Result<Html<String>, (StatusCode, String)> {
    let random_cat_gif_html = match RandomCatGifTera::build_html(tenor_results) {
        Ok(random_cat_gif_html) => random_cat_gif_html,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    Ok(Html(random_cat_gif_html))
}
