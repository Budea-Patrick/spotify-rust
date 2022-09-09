use std::fs::File;
use std::io::Write;
use urlencoding::encode;

const CLIENT_ID: &str = "422489842a194b639397736aefc2a55a";
const _CLIENT_SECRET: &str = "cad6f1f0d0724e50b0ebf9d457142d65";
const REDIRECT_URI: &str = "http://localhost:63342/spotify-rust/simple_web_request/index.html?_ijt=dr265ocjvqn7enoo2e2lajve0d&_ij_reload=RELOAD_ON_SAVE";
const AUTHORIZE: &str = "https://accounts.spotify.com/authorize";

fn create_url() -> String {
    let mut url = AUTHORIZE.to_string();
    url.push_str("?client_id=");
    url.push_str(&CLIENT_ID);
    url.push_str("&response_type=code");
    url.push_str("&redirect_uri=");
    url.push_str(&encode(&REDIRECT_URI));
    url.push_str("&show_dialog=true");
    url.push_str("&scope=user-read-private user-read-email user-modify-playback-state user-read-playback-position user-library-read streaming user-read-playback-state user-read-recently-played playlist-read-private");

    return url;
}

fn process_url(url: &String) {
    let mut file = File::create("link.txt").expect("Cannot create file");
    open::that(&url).expect("Cannot open URL");
    file.write_all(url.as_ref()).expect("Cannot write");
    println!("{}", url);
}

async fn get_cat_fact(url: &String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).send().await?.text().await?;

    Ok(body)
}

#[tokio::main]
async fn main() {
    let url = create_url();
    process_url(&url);
    let fact = get_cat_fact(&url).await;
    println!("fact = {:#?}", fact);
}
