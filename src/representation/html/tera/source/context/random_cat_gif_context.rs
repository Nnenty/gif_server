use tera::Context;

use crate::{
    representation::html::tera::source::constants::{TERA_GIF_DESCRIPTION_VAR, TERA_GIF_URL_VAR},
    tenor::TenorResults,
};

pub fn build_random_cat_gif_context(tenor_results: TenorResults) -> Context {
    let mut context = Context::new();
    context.insert(TERA_GIF_URL_VAR, tenor_results.get_gif_url());
    context.insert(
        TERA_GIF_DESCRIPTION_VAR,
        tenor_results.content_description(),
    );

    context
}
