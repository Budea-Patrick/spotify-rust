use core::fmt;
use std::fmt::{Formatter};

use serde::Deserialize;

use crate::entities::image::Image;
use crate::external_url::ExternalUrls;

// wrapper type
#[derive(Deserialize, Debug)]
struct AlbumArtist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    name: String,
    external_urls: ExternalUrls,
    images: Vec<Image>,
    artists: Vec<AlbumArtist>,
    release_date: String,
    total_tracks: u8,
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Album name: {}\nYear: {}", self.name, self.release_date)
    }
}

impl Album {
    pub(crate) fn get_album_image(&self) -> &String {
        let image = self.images.first().unwrap();
        return &image.url;
    }

    pub(crate) fn print_album_info(&self) -> String {
        let album_artist = self.artists.first().unwrap();
        let text = format!("Album name: {}\nArtist: {}\nRelease date: {}\nTotal tracks: {}\nExternal link: {}",
                           self.name, album_artist.name, self.release_date, self.total_tracks,self.external_urls.spotify
        );
        return text;
    }
}

impl fmt::Display for AlbumArtist {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.external_urls.spotify)
    }
}