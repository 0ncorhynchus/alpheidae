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

impl TokenKeys {
    pub fn new(consumer_keys: KeyPair) -> Self {
        Self {
            consumer_keys,
            oauth_tokens: None,
        }
    }

    pub fn oauth_tokens(mut self, oauth_tokens: KeyPair) -> Self {
        self.oauth_tokens = Some(oauth_tokens);
        self
    }
}
