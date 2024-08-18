use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TenorMp4Format {
    pub url: String,
}
