use core::fmt;
use std::fmt::Formatter;

use serde::Deserialize;

use crate::entities::album::Album;
use crate::external_url::ExternalUrls;

// wrapper type
#[derive(Deserialize, Debug)]
struct TrackArtist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    name: String,
    popularity: u32,
    pub(crate) album: Album,
    artists: Vec<TrackArtist>,
    external_urls: ExternalUrls,
    duration_ms: u32,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let track_artist = self.artists.first().unwrap();
        write!(f, "{} - {} - {} - {} - {} - {}", self.name, self.popularity, self.album, self.external_urls.spotify, self.duration_ms, track_artist)
    }
}

impl fmt::Display for TrackArtist {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.external_urls.spotify)
    }
}

impl Track {
    pub(crate) fn print_track_info(&self) -> String {
        let track_artist = self.artists.first().unwrap();
        let text = format!("Track name: {}\n{}\nArtist: {}\nPopularity: {}\nDuration: {} seconds\nExternal link: {}",
                           self.name, self.album, track_artist.name, self.popularity, self.duration_ms/1000, self.external_urls.spotify
        );
        return text;
    }
}