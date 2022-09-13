use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub(crate) struct Artist {
    name: String,
    external_urls: String,
}
