use core::fmt;
use serde::Deserialize;
use std::fmt::Formatter;

use crate::entities::image::Image;
use crate::external_url::ExternalUrls;

#[derive(Deserialize, Debug)]
pub struct Artist {
    name: String,
    external_urls: ExternalUrls,
    images: Vec<Image>,
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.external_urls.spotify)
    }
}

impl Artist {
    pub(crate) fn get_artist_image(&self) -> &String {
        let image = self.images.first().unwrap();
        return &image.url;
    }
}
