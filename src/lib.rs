mod error;
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
    id: String,
    url: String,
    place_type: String,
    name: String,
    full_name: String,
    country_code: String,
    country: String,
    bounding_box: BoundingBox,
    // attributes: Object,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BoundingBox {
    coordinates: [[[f64; 2]; 2]; 2],
    r#type: String,
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
    created_at: String,
    id: u64,
    text: String,
    entities: Entity,
    source: String,
    truncated: bool,
    in_reply_to_status_id: Option<u64>,
    in_reply_to_user_id: Option<u64>,
    in_reply_to_screen_name: Option<String>,
    user: User,
    coordinates: Option<Coordinate>,
    place: Option<Place>,
    is_quote_status: bool,
    retweet_count: u64,
    favorite_count: u64,
    favorited: Option<bool>,
    retweeted: bool,
    lang: Option<String>,
    quoted_status_id: Option<u64>,
    quoted_status: Option<Box<Tweet>>,
    retweeted_status: Option<Box<Tweet>>,
    quote_count: Option<u64>,
    reply_count: Option<u64>,
    extended_entities: Option<ExtendedEntity>,
    possibly_sensitive: Option<bool>,
    filter_level: Option<String>,
    matching_rules: Option<Vec<Rule>>,
}
