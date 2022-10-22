use serde::Deserialize;

use crate::external_url::ExternalUrls;

#[derive(Deserialize, Debug)]
pub(crate) struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
