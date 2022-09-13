use crate::header::{HeaderMap, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, ACCEPT};
use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::format;
use crate::extract_token;

const JSON_CONTENT_TYPE: &str = "application/json";

#[derive(Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Deserialize, Debug)]
struct APIResponse {
    albums: Items<Album>,
}

fn print_albums(albums : Vec<&Album>) {
    for album in albums {
        println!("{} - {}", album.name, album.external_urls.spotify);
    }
}

pub async fn get_albums(artist : &str) {
    let url = format!(
        "https://api.spotify.com/v1/search?q={artist}&type=album",
        artist = artist);
    let client = Client::new();
    let response = client.get(url)
        .header(AUTHORIZATION, format!("Bearer {}", extract_token().await))
        .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        .header(ACCEPT, JSON_CONTENT_TYPE)
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            let parsed = response.json::<APIResponse>().await.unwrap();
            print_albums(parsed.albums.items.iter().collect())
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Token expired");
        }
        _ => {
            panic!("Unexpected error");
        }
    }
}