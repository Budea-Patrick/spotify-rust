use serde::Deserialize;

use crate::entities::album::Album;
use crate::entities::artist::Artist;

#[derive(Deserialize, Debug)]
pub(crate) struct Items<T> {
    pub(crate) items: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct AlbumResponse {
    pub(crate) albums: Items<Album>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ArtistResponse {
    pub(crate) artists: Items<Artist>,
}
