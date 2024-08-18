use axum::response::Html;
use reqwest::StatusCode;
use tera::{Context, Tera};
use tracing::{event, Level};

use crate::tenor::TenorResults;

pub async fn get_random_cat_gif_html(
    tenor_results: TenorResults,
) -> Result<Html<String>, (StatusCode, String)> {
    let tera = match Tera::new("src/public/**") {
        Ok(tera) => tera,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    event!(
        Level::DEBUG,
        "url = {} desc = {}",
        tenor_results.get_gif_url(),
        tenor_results.content_description()
    );

    let mut context = Context::new();
    context.insert("gif_url", tenor_results.get_gif_url());
    context.insert("gif_description", tenor_results.content_description());

    let random_cat_gif_html = match tera.render("random_cat_gif.html", &context) {
        Ok(s) => s,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    Ok(Html(random_cat_gif_html))
}
