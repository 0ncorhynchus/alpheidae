use alpheidae::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
}

#[actix_rt::test]
async fn get_tweet() {
    let config: Config = envy::from_env().unwrap();
    let consumer_keys = KeyPair::new(config.consumer_key, config.consumer_secret);
    let oauth_tokens = KeyPair::new(config.access_token, config.access_token_secret);
    let tokens = TokenKeys::new(consumer_keys).oauth_tokens(oauth_tokens);

    let id = 210462857140252672;
    let tweet = statuses::show(&tokens, id).send().await.unwrap();
    assert_eq!(id, tweet.id);
}
