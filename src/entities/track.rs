use serde::Deserialize;

use crate::entities::album::Album;
use crate::external_url::ExternalUrls;

#[derive(Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}