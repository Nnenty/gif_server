use serde::Deserialize;

use super::media_format::TenorMedia;

#[derive(Deserialize, Debug)]
struct TenorResult {
    content_description: String,
    media: TenorMedia,
}

#[derive(Deserialize, Debug)]
pub struct TenorResults {
    results: Vec<TenorResult>,
}

impl TenorResults {
    pub fn get_gif_url(&self) -> &str {
        &self.results[0].media.0[0].gif.url
    }
    pub fn content_description(&self) -> &str {
        &self.results[0].content_description
    }
}
