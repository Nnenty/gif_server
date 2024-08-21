use tera::Context;

pub fn build_random_cat_gif_context(gif_url: &str, gif_description: &str) -> Context {
    let mut context = Context::new();
    context.insert("gif_url", gif_url);
    context.insert("gif_description", gif_description);

    context
}
