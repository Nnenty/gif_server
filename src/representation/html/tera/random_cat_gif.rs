use std::sync::LazyLock;

use tera::{Context, Tera};

use crate::tenor::TenorResults;

use super::source::{context::build_random_cat_gif_context, templates::RANDOM_CAT_GIF_TEMPLATE};

pub struct RandomCatGifTera;

impl RandomCatGifTera {
    pub fn build_html(tenor_results: TenorResults) -> Result<String, tera::Error> {
        let template = get_template();
        let context = get_context(tenor_results);

        let random_cat_gif_html = template.render("random_cat_gif.html", &context)?;

        Ok(random_cat_gif_html)
    }
}

fn get_template() -> &'static LazyLock<Tera> {
    &RANDOM_CAT_GIF_TEMPLATE
}
fn get_context(tenor_results: TenorResults) -> Context {
    build_random_cat_gif_context(tenor_results)
}
