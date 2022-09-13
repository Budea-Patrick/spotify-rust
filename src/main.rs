mod token;
mod entities;
mod api_response;

use std::collections::HashMap;
use std::fmt::format;

use reqwest::Client;
use reqwest::header;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use crate::entities::get_albums;

use crate::header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, HeaderMap};
use crate::token::extract_token;

#[tokio::main]
async fn main() {
    get_albums("Durbatuluk", Client::new(), extract_token().await).await;
}
