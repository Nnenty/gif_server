use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Mp4 {
    pub url: String,
}
