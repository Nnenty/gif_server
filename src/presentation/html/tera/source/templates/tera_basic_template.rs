use std::{process, sync::LazyLock};
use tera::Tera;
use tracing::{event, Level};

pub static TERA_TEMPLATES: LazyLock<Tera> = LazyLock::new(|| match Tera::new("public/**") {
    Ok(tera) => tera,
    Err(err) => {
        event!(Level::DEBUG, "Fatal error {err}");
        process::exit(1)
    }
});
