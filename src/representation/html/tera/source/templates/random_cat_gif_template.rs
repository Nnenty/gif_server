use std::{process, sync::LazyLock};
use tera::Tera;
use tracing::{event, Level};

use crate::representation::html::tera::source::constants::HTML_TEMPLATES_PATH_FOR_TERA;

pub static RANDOM_CAT_GIF_TEMPLATE: LazyLock<Tera> =
    LazyLock::new(|| match Tera::new(HTML_TEMPLATES_PATH_FOR_TERA) {
        Ok(tera) => tera,
        Err(err) => {
            event!(Level::DEBUG, "Fatal error {err}");
            process::exit(1)
        }
    });
