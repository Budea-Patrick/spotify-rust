/// Lightweight RUST based API call to Spotify's API\
/// Using client credentials flow (see https://developer.spotify.com/documentation/general/guides/authorization/client-credentials/)
/// API call made using asynchronous runtime using tokio
/// See Cargo.toml for crate usage
use crate::header::{HeaderMap, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, ACCEPT};
use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::format;

const CLIENT_ID: &str = "422489842a194b639397736aefc2a55a";
const CLIENT_SECRET: &str = "cad6f1f0d0724e50b0ebf9d457142d65";
const REQUEST_URL: &str = "https://accounts.spotify.com/api/token";
const URL_ENCODED_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";
const JSON_CONTENT_TYPE: &str = "application/json";

#[derive(Deserialize, Debug)]
struct ResponseJson {
    access_token: String,
    token_type: String,
    expires_in: u16,
}

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
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
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

// Spotify requires base64 encoding of client credentials when making API calls
fn encode_client_credentials() -> String {
    let mut encoded = format!("{}:{}", CLIENT_ID, CLIENT_SECRET);
    encoded = base64::encode(encoded);
    return encoded;
}

// Generate authorization header
fn client_authorization(client_credentials: String) -> String {
    let authorization = format!("Basic {}", client_credentials);
    return authorization;
}

// Generate request headers
fn generate_header(authorization_string: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization_string.parse().unwrap());

    // The API call requires the content length to be present
    headers.insert(CONTENT_LENGTH, HeaderValue::from(0));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(URL_ENCODED_CONTENT_TYPE));
    return headers;
}

// TODO Replace hardcoded values
fn set_parameters() -> HashMap<&'static str, &'static str> {
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    return params;
}

// Make POST call and return response as deserialized JSON
async fn get_response_json() -> ResponseJson {
    let headers = generate_header(client_authorization(encode_client_credentials()));

    let client = Client::new();
    let resp = client
        .post(REQUEST_URL)
        .query(&set_parameters())
        .headers(headers)
        .send()
        .await
        .unwrap();
    match resp.status() {
        reqwest::StatusCode::OK => {
            return resp.json::<ResponseJson>().await.unwrap();
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Need new token");
        }
        _ => {
            panic!("Error at making API call");
        }
    }
}

fn extract_token(response_json: ResponseJson) -> String {
    return response_json.access_token;
}

fn print_albums(albums : Vec<&Album>) {
    for album in albums {
        println!("{} - {}", album.name, album.external_urls.spotify);
    }
}

async fn get_albums(artist : &str) {
    let url = format!(
        "https://api.spotify.com/v1/search?q={artist}&type=album",
        artist = artist);
    let client = Client::new();
    let response = client.get(url)
        .header(AUTHORIZATION, format!("Bearer {}", extract_token(get_response_json().await)))
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

#[tokio::main]
async fn main() {
    //println!("{}", extract_token(get_response_json().await));
    // println!("{:#?}", get_response_json().await);
    get_albums("Durbatuluk").await;
}
