use alpheidae::*;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;

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
    let file: String = args.next().expect("An argument is required: String");
    let mut file = File::open(file).unwrap();
    let mut buffer = Vec::new();
    let len = file.read_to_end(&mut buffer).unwrap();

    let res = media::upload_init(&tokens, len as u32, "image/png".to_string())
        .send()
        .await
        .unwrap();
    let media_id = res.media_id;

    for (i, chunk) in buffer.chunks(5 * 1024 * 1024).enumerate() {
        media::upload_append(&tokens, media_id, chunk.to_vec(), i as u16)
            .send()
            .await
            .unwrap();
    }

    let res = media::upload_finalize(&tokens, media_id)
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
}
