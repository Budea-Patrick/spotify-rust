use reqwest::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use crate::api_response::APIResponse;
use crate::entities::album::Album;

mod album;
mod artist;
mod track;

const JSON_CONTENT_TYPE: &str = "application/json";

pub async fn get_albums(artist : &str, client : Client, token : String) {
    let url = format!(
        "https://api.spotify.com/v1/search?q={artist}&type=album",
        artist = artist);
    let response = client.get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        .header(ACCEPT, JSON_CONTENT_TYPE)
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            let parsed = response.json::<APIResponse<Album>>().await.unwrap();
            let albums = parsed.get_items().get_items().iter().collect();
            for album in albums {
                album.print_album();
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Token expired");
        }
        _ => {
            panic!("Unexpected error");
        }
    }
}