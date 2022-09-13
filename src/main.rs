mod token;
mod entities;

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
    //println!("{}", extract_token(get_response_json().await));
    // println!("{:#?}", get_response_json().await);
    get_albums("Durbatuluk").await;
}
