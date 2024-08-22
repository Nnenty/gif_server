use axum::response::Html;

use super::tera::source::{context::build_search_gif_context, templates::TERA_TEMPLATES};

pub async fn get_search_gif_html<'a>(
    gifs_url: Option<Vec<&'a str>>,
    gifs_description: Option<Vec<&'a str>>,
    parameter_value: Option<&str>,
) -> Result<Html<String>, tera::Error> {
    let context = build_search_gif_context(gifs_url, gifs_description, parameter_value);
    let template = &TERA_TEMPLATES;

    let searh_gif = match template.render("search_gif.html", &context) {
        Ok(searh_gif) => searh_gif,
        Err(err) => return Err(err),
    };

    Ok(Html(searh_gif))
}
