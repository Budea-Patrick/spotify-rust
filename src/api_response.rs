use serde::Deserialize;
use crate::entities::album::Album;

#[derive(Deserialize, Debug)]
pub(crate) struct Items<T> {
    pub(crate) items: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct APIResponse {
    pub(crate) albums: Items<Album>,
}
