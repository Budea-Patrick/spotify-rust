use serde::Deserialize;
use core::fmt;
use std::fmt::Formatter;

use crate::external_url::ExternalUrls;

#[derive(Deserialize, Debug)]
pub struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.external_urls.spotify)
    }
}