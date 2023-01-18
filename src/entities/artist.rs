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
    genres: Vec<String>,
    popularity: u8,
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

    pub(crate) fn print_artist_info(&self) -> String {
        let mut genres = String::new();
        for genre in &self.genres {
            genres.push_str(genre.as_str());
            genres.push_str(", ");
        }
        genres.pop();
        genres.pop();
        let text = format!("Artist: {}\nGenres: {}\nPopularity: {}\nExternal link: {}",
                           self.name, genres, self.popularity, self.external_urls.spotify
        );
        return text;
    }
}
