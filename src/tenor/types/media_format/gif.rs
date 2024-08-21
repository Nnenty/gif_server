use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Gif {
    pub url: String,
}
