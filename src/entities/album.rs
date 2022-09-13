use serde::Deserialize;
use crate::entities::artist::Artist;
use crate::external_url::ExternalUrls;

#[derive(Deserialize, Debug)]
pub(crate) struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

impl Album {
    pub(crate) fn print_album(&self) {
        println!("{} - {}", self.name, self.external_urls.spotify);
    }
}