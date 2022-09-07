use urlencoding::encode;
#[tokio::main]
async fn main() {
    let fact = get_cat_fact().await;

    println!("fact = {:#?}", fact);
}

async fn get_cat_fact() -> Result<String, Box<dyn std::error::Error>> {
    let client_id = String::from("422489842a194b639397736aefc2a55a");
    let _client_secret = String::from("cad6f1f0d0724e50b0ebf9d457142d65");
    let redirect_uri = String::from("http://localhost:8888/callback");
    let authorize = String::from("https://accounts.spotify.com/authorize");

    let mut url = authorize.to_string();
    url.push_str("?client_id=");
    url.push_str(&client_id);
    url.push_str("&response_type=code");
    url.push_str("&redirect_uri=");
    url.push_str(&encode(&redirect_uri));
    url.push_str("&show_dialog=true");
    url.push_str("&scope=user-read-private user-read-email user-modify-playback-state user-read-playback-position user-library-read streaming user-read-playback-state user-read-recently-played playlist-read-private");

    println!("{}", url);

    let client = reqwest::Client::new();

    let body = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
