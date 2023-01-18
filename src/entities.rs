use std::io::{Read};
use new_image::{DynamicImage, EncodableLayout, load_from_memory};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, StatusCode};

use crate::api_response::{AlbumResponse, ArtistResponse, TrackResponse};
use crate::entities::album::Album;
use crate::entities::artist::Artist;
use crate::entities::track::Track;

pub mod album;
pub(crate) mod artist;
mod image;
pub(crate) mod track;

const JSON_CONTENT_TYPE: &str = "application/json";


pub fn get_album_response(album: &str, token: String) -> Vec<Album> {
    let url = format!(
        "https://api.spotify.com/v1/search?q={album}&type=album&limit=3",
        album = album);
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        .header(ACCEPT, JSON_CONTENT_TYPE)
        .send()
        .unwrap();
    match response.status() {
        StatusCode::OK => {
            // println!("{}", response.text().unwrap());
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

pub fn get_artist_response(artist: &str, token: String) -> Vec<Artist> {
    let url = format!(
        "https://api.spotify.com/v1/search?q={artist}&type=artist&limit=1",
        artist = artist
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
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
            return artists;
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

pub fn get_track_response(track: &str, token: String) -> Vec<Track> {
    let url = format!(
        "https://api.spotify.com/v1/search?q={track}&type=track&limit=3",
        track = track
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        .header(ACCEPT, JSON_CONTENT_TYPE)
        .send()
        .unwrap();
    match response.status() {
        StatusCode::OK => {
            let parsed = response.json::<TrackResponse>().unwrap();
            let elements = parsed.tracks;
            let tracks: Vec<Track> = elements.items;
            return tracks;
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