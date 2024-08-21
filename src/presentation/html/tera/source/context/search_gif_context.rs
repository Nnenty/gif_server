use tera::Context;

pub fn build_search_gif_context(
    gifs_url: Option<Vec<&str>>,
    gifs_description: Option<Vec<&str>>,
) -> Context {
    let mut context: Context = Context::new();

    if gifs_url.is_some() && gifs_description.is_some() {
        context.insert("gifs", &gifs_url);
        context.insert("gifs_description", &gifs_description)
    }

    context
}
