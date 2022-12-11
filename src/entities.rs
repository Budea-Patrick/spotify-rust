use reqwest::{Client, StatusCode};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::api_response::{AlbumResponse, ArtistResponse};
use crate::entities::album::Album;
use crate::entities::artist::Artist;

pub mod album;
pub(crate) mod artist;
mod track;
mod image;

const JSON_CONTENT_TYPE: &str = "application/json";

pub fn get_album_response(artist: &str, token: String) -> Vec<Album> {
    let url = format!(
        "https://api.spotify.com/v1/search?q={artist}&type=album",
        artist = artist);
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        .header(ACCEPT, JSON_CONTENT_TYPE)
        .send()
        .unwrap();
    match response.status() {
        StatusCode::OK => {
            let parsed = response.json::<AlbumResponse>().unwrap();
            let elements = parsed.albums;
            let albums: Vec<Album> = elements.items;
            return albums;
        }
        StatusCode::UNAUTHORIZED => {
            println!("Token expired");
            panic!("Token expired");
        }
        _ => {
            panic!("Unexpected error");
        }
    }
}

pub fn get_artist_response(artist: &str, token: String) {
    let url = format!(
        "https://api.spotify.com/v1/search?q={artist}&type=artist",
        artist = artist);
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        .header(ACCEPT, JSON_CONTENT_TYPE)
        .send()
        .unwrap();
    match response.status() {
        StatusCode::OK => {
            let parsed = response.json::<ArtistResponse>().unwrap();
            let elements = parsed.artists;
            let artists: Vec<Artist> = elements.items;
            for artist in artists {
                println!("{}", artist);
            }
            //return artists;
        }
        StatusCode::UNAUTHORIZED => {
            println!("Token expired");
            panic!("Token expired");
        }
        _ => {
            panic!("Unexpected error");
        }
    }
}
