/// Lightweight RUST based API call to Spotify's API\
/// Using client credentials flow (see https://developer.spotify.com/documentation/general/guides/authorization/client-credentials/)
/// API call made using asynchronous runtime using tokio
/// See Cargo.toml for crate usage
use crate::header::{HeaderMap, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::Client;
use std::collections::HashMap;

const CLIENT_ID: &str = "422489842a194b639397736aefc2a55a";
const CLIENT_SECRET: &str = "cad6f1f0d0724e50b0ebf9d457142d65";
const REQUEST_URL: &str = "https://accounts.spotify.com/api/token";
const POST_TYPE: &str = "application/x-www-form-urlencoded";

// Spotify requires base64 encoding of client credentials when making API calls
fn encode_client_credentials() -> String {
    let mut encoded = CLIENT_ID.to_string();
    encoded.push_str(":");
    encoded.push_str(&CLIENT_SECRET);
    encoded = base64::encode(encoded);
    return encoded;
}

// Generate authorization header
fn client_authorization(client_credentials: String) -> String {
    let mut authorization = "Basic ".to_string();
    authorization.push_str(&client_credentials);
    return authorization;
}

// Generate request headers
fn generate_header(authorization_string: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization_string.parse().unwrap());

    // The API call requires the content length to be present
    headers.insert(CONTENT_LENGTH, HeaderValue::from(0));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(POST_TYPE));
    return headers;
}

// Make POST call and return response encapsulated in Box, also checking for errors
// TODO Replace hardcoded values
async fn get_response() -> Result<String, Box<dyn std::error::Error>> {
    let headers = generate_header(client_authorization(encode_client_credentials()));

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");

    let client = Client::new();
    let body = client
        .post(REQUEST_URL)
        .query(&params)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let response = get_response().await;
    println!("response = {:#?}", response);
}
