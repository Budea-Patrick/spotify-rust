use std::collections::HashMap;
use std::fmt::format;

use reqwest::Client;
use reqwest::header;
use reqwest::header::HeaderValue;
use serde::Deserialize;

use crate::header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, HeaderMap};

const CLIENT_ID: &str = "422489842a194b639397736aefc2a55a";
const CLIENT_SECRET: &str = "cad6f1f0d0724e50b0ebf9d457142d65";
const REQUEST_URL: &str = "https://accounts.spotify.com/api/token";
const URL_ENCODED_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

#[derive(Deserialize, Debug)]
struct ResponseJson {
    access_token: String,
    token_type: String,
    expires_in: u16,
}

fn encode_client_credentials() -> String {
    let mut encoded = format!("{}:{}", CLIENT_ID, CLIENT_SECRET);
    encoded = base64::encode(encoded);
    return encoded;
}

fn client_authorization(client_credentials: String) -> String {
    let authorization = format!("Basic {}", client_credentials);
    return authorization;
}

fn generate_header(authorization_string: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization_string.parse().unwrap());

    // The API call requires the content length to be present
    headers.insert(CONTENT_LENGTH, HeaderValue::from(0));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(URL_ENCODED_CONTENT_TYPE));
    return headers;
}

fn set_parameters() -> HashMap<&'static str, &'static str> {
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    return params;
}

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

pub async fn extract_token() -> String {
    return get_response_json().await.access_token;
}