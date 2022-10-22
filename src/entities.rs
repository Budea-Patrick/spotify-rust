use reqwest::{Client, StatusCode};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::api_response::AlbumResponse;
use crate::entities::album::Album;

pub mod album;
mod artist;
mod track;
mod image;

const JSON_CONTENT_TYPE: &str = "application/json";

pub async fn get_album_response(artist: &str, client: Client, token: String) {
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
        StatusCode::OK => {
            let parsed = response.json::<AlbumResponse>().await.unwrap();
            let elements = parsed.albums;
            let albums: Vec<Album> = elements.items;
            for album in albums {
                println!("{}", album);
                album.print_covers();
            }
        }
        StatusCode::UNAUTHORIZED => {
            println!("Token expired");
        }
        _ => {
            panic!("Unexpected error");
        }
    }
}
