use core::fmt;
use std::fmt::Formatter;

use serde::Deserialize;

use crate::entities::artist::Artist;
use crate::entities::image::Image;
use crate::external_url::ExternalUrls;

#[derive(Deserialize, Debug)]
pub(crate) struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
    images: Vec<Image>,
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Album {
    pub(crate) fn print_covers(&self) {
        for image in &self.images {
            println!("{}", image);
        }
    }
}