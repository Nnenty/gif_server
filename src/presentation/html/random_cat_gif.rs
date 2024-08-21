use axum::response::Html;

use super::tera::source::context::build_random_cat_gif_context;
use super::tera::source::templates::TERA_TEMPLATES;

pub async fn get_random_cat_gif_html(
    gif_url: &str,
    gif_description: &str,
) -> Result<Html<String>, tera::Error> {
    let context = build_random_cat_gif_context(gif_url, gif_description);
    let template = &TERA_TEMPLATES;

    let random_cat_gif_html = match template.render("random_cat_gif.html", &context) {
        Ok(random_cat_gif_html) => random_cat_gif_html,
        Err(err) => return Err(err),
    };

    Ok(Html(random_cat_gif_html))
}
