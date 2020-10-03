use alpheidae::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    consumer_key: String,
    consumer_secret: String,
    callback_url: String,
}

#[actix_rt::test]
async fn request_token() {
    let config: Config = envy::from_env().unwrap();
    let consumer_keys = KeyPair::new(config.consumer_key, config.consumer_secret);

    let response = oauth::request_token(&consumer_keys, config.callback_url)
        .x_auth_access_type(oauth::AccessType::Read)
        .send()
        .await
        .unwrap();

    assert!(response.oauth_callback_confirmed);
}
