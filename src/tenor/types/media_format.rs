use serde::Deserialize;

mod gif;

use gif::Gif;

#[derive(Deserialize, Debug)]
pub struct Media(pub Vec<Format>);

#[derive(Deserialize, Debug)]
pub struct Format {
    pub gif: Gif,
}
