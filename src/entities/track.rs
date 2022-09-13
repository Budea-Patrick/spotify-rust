use serde::Deserialize;
use crate::entities::album::Album;

#[derive(Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: String,
}