extern crate core;


use std::env;
use reqwest::Client;
use reqwest::header;
use serde::Deserialize;

use crate::entities::get_album_response;
use crate::token::extract_token;

mod token;
mod entities;
mod api_response;
mod external_url;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Command line argument error");
    }

    let artist_name = &args[1];
    get_album_response(artist_name, Client::new(), extract_token().await).await;
}
