mod error;
mod macros;
pub mod oauth;
pub mod statuses;
mod utils;

pub use error::Error;
use serde::Deserialize;

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

// TODO
#[derive(Deserialize, Clone, Debug)]
pub struct User {}

pub type Coordinate = [f64; 2];

#[derive(Deserialize, Clone, Debug)]
pub struct Place {
    pub id: String,
    pub url: String,
    pub place_type: String,
    pub name: String,
    pub full_name: String,
    pub country_code: String,
    pub country: String,
    pub bounding_box: BoundingBox,
    // pub attributes: Object,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BoundingBox {
    pub coordinates: [[[f64; 2]; 2]; 2],
    pub r#type: String,
}

// TODO
#[derive(Deserialize, Clone, Debug)]
pub struct Entity {}

// TODO
#[derive(Deserialize, Clone, Debug)]
pub struct ExtendedEntity {}

// TODO
#[derive(Deserialize, Clone, Debug)]
pub struct Rule {}

#[derive(Deserialize, Clone, Debug)]
pub struct Tweet {
    pub created_at: String,
    pub id: u64,
    pub text: String,
    pub entities: Entity,
    pub source: String,
    pub truncated: bool,
    pub in_reply_to_status_id: Option<u64>,
    pub in_reply_to_user_id: Option<u64>,
    pub in_reply_to_screen_name: Option<String>,
    pub user: User,
    pub coordinates: Option<Coordinate>,
    pub place: Option<Place>,
    pub is_quote_status: bool,
    pub retweet_count: u64,
    pub favorite_count: u64,
    pub favorited: Option<bool>,
    pub retweeted: bool,
    pub lang: Option<String>,
    pub quoted_status_id: Option<u64>,
    pub quoted_status: Option<Box<Tweet>>,
    pub retweeted_status: Option<Box<Tweet>>,
    pub quote_count: Option<u64>,
    pub reply_count: Option<u64>,
    pub extended_entities: Option<ExtendedEntity>,
    pub possibly_sensitive: Option<bool>,
    pub filter_level: Option<String>,
    pub matching_rules: Option<Vec<Rule>>,
}
