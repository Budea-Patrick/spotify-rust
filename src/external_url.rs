use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct ExternalUrls {
    pub(crate) spotify: String,
}