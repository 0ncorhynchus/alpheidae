use alpheidae::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
}

#[actix_rt::main]
async fn main() {
    let config: Config = envy::from_env().unwrap();
    let consumer_keys = KeyPair::new(config.consumer_key, config.consumer_secret);
    let oauth_tokens = KeyPair::new(config.access_token, config.access_token_secret);
    let tokens = TokenKeys::new(consumer_keys).oauth_tokens(oauth_tokens);

    let mut args = std::env::args();
    args.next();
    let id: u64 = args
        .next()
        .expect("An argument is required: id")
        .parse()
        .expect("Invalid argument is given. expected u64");

    let resposne = statuses::destroy(&tokens, id).send().await.unwrap();
    println!("{:?}", resposne);
}
