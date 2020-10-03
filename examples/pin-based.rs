use serde::Deserialize;
use std::io::{self, Write};
use twitter_api::*;

#[derive(Deserialize)]
struct Config {
    consumer_key: String,
    consumer_secret: String,
}

fn read_pin(url: String) -> u32 {
    println!("Please open: {}", url);
    print!("then, type the PIN number: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().parse().expect("Invalid input.")
}

#[actix_rt::main]
async fn main() {
    let config: Config = envy::from_env().unwrap();
    let consumer_keys = KeyPair::new(config.consumer_key, config.consumer_secret);
    let callback_url = "oob".to_string();

    let response = oauth::request_token(&consumer_keys, callback_url)
        .x_auth_access_type(oauth::AccessType::Read)
        .send()
        .await;
    assert!(response.oauth_callback_confirmed);

    let pin = read_pin(response.get_redirect_url());
    let response = oauth::access_token(&consumer_keys, response.oauth_token, pin.to_string())
        .send()
        .await;

    println!("Hello, {}!", response.screen_name);

    let tokens = TokenKeys::new(consumer_keys).oauth_tokens(KeyPair::new(
        response.oauth_token,
        response.oauth_token_secret,
    ));

    oauth::invalidate_token(&tokens).send().await;
}
