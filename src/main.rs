use std::collections::HashMap;

use reqwest::header;
use reqwest::header::HeaderValue;
use reqwest::{Body, Client};

use crate::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};

const CLIENT_ID: &str = "422489842a194b639397736aefc2a55a";
const CLIENT_SECRET: &str = "cad6f1f0d0724e50b0ebf9d457142d65";

async fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    let mut encoded = CLIENT_ID.to_string();
    encoded.push_str(":");
    encoded.push_str(&CLIENT_SECRET);
    encoded = base64::encode(encoded);

    let mut header_construct = "Basic ".to_string();
    header_construct.push_str(&encoded);

    let header_value = header_construct.as_str();

    println!("{}", header_value);

    let mut headers = header::HeaderMap::new();
    headers.insert(AUTHORIZATION, header_value.parse().unwrap());
    headers.insert(CONTENT_LENGTH, HeaderValue::from(0));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let grant_string = "grant_type=client_credentials".to_string();
    println!("{}", grant_string);

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");

    let client = Client::new();
    let body = client
        .post("https://accounts.spotify.com/api/token")
        .body(Body::from(grant_string))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let fact = get_token().await;
    println!("fact = {:#?}", fact);
}
