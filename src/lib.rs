pub mod oauth;
mod utils;

#[derive(Clone)]
pub struct KeyPair {
    key: String,
    secret: String,
}

impl KeyPair {
    pub fn new(key: String, secret: String) -> Self {
        Self { key, secret }
    }
}

#[derive(Clone)]
pub struct TokenKeys {
    consumer_keys: KeyPair,
    oauth_tokens: Option<KeyPair>,
}
