use serde::Deserialize;
// #[path = "artist.rs"]
use crate::entities::artist::Artist;

#[derive(Deserialize, Debug)]
pub(crate) struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: String,
}

impl Album {
    pub(crate) fn print_album(&self) {
        println!("{} - {}", self.name, self.external_urls);
    }
}