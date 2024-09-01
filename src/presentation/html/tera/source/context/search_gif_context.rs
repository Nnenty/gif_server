use tera::Context;

pub fn build_search_gif_context(
    gifs_url: Option<Vec<&str>>,
    gifs_description: Option<Vec<&str>>,
    parameter_value: Option<&str>,
) -> Context {
    let mut context: Context = Context::new();

    if gifs_url.is_some() && gifs_description.is_some() && parameter_value.is_some() {
        let mut params = String::new();

        // replace all whitespace characters with symbols '+'
        parameter_value
            // unwrap because we made sure that the value ​​is Some
            .unwrap()
            .char_indices()
            .for_each(|(_, letter)| {
                if letter.cmp(&' ').is_eq() {
                    params.push('+');
                } else {
                    params.push(letter);
                }
            });

        context.insert("gifs", &gifs_url);
        context.insert("gifs_description", &gifs_description);
        context.insert("user_query", &parameter_value);
        context.insert("params", &params);
    }

    context
}
