use serde::Deserialize;

mod gif;
mod mp4;
mod webp;

use gif::Gif;
use mp4::Mp4;
use webp::Webp;

#[derive(Deserialize, Debug)]
pub struct Media(pub Vec<Format>);

#[derive(Deserialize, Debug)]
pub struct Format {
    pub gif: Gif,
    pub webp: Webp,
    pub mp4: Mp4,
}
