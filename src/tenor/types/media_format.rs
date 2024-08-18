use serde::Deserialize;

mod gif;
mod mp4;
mod webp;

use gif::TenorGifFormat;
use mp4::TenorMp4Format;
use webp::TenorWebpFormat;

#[derive(Deserialize, Debug)]
pub struct TenorMedia(pub Vec<TenorFormat>);

#[derive(Deserialize, Debug)]
pub struct TenorFormat {
    pub gif: TenorGifFormat,
    pub webp: TenorWebpFormat,
    pub mp4: TenorMp4Format,
}
