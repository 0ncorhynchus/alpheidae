use actix_web::client::ClientRequest;
use actix_web::http::header::AUTHORIZATION;
use chrono::{offset::Local, DateTime};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt;

pub trait OAuthRequest<K, V> {
    fn oauth(self, params: Vec<(K, V)>) -> Self;
}

impl<K, V> OAuthRequest<K, V> for ClientRequest
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn oauth(self, params: Vec<(K, V)>) -> Self {
        let mut params: Vec<_> = params
            .into_iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect();
        params.sort();

        let token = format!("OAuth {}", params.join(", "));
        self.header(AUTHORIZATION, token)
    }
}

pub fn percent_encode(input: &str) -> String {
    const FRAGMENTS: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~');
    utf8_percent_encode(input, FRAGMENTS).to_string()
}

pub fn gen_signature_key(consumer_secret: &str, oauth_token_secret: &str) -> String {
    format!(
        "{}&{}",
        percent_encode(consumer_secret),
        percent_encode(oauth_token_secret)
    )
}

fn gen_signature(key: String, url: &str, params: &str) -> String {
    let signature_data = format!(
        "{}&{}&{}",
        percent_encode("POST"),
        percent_encode(url),
        percent_encode(params)
    );
    percent_encode(&base64::encode(&hmacsha1::hmac_sha1(
        key.as_bytes(),
        signature_data.as_bytes(),
    )))
}

pub fn write_signature(
    key: String,
    url: &str,
    params: &mut Vec<(&'static str, String)>,
    query: &[(&'static str, String)],
) {
    let mut params_str = params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>();
    for (k, v) in query {
        params_str.push(format!("{}={}", k, v));
    }
    params_str.sort();
    let params_str = &params_str.join("&");

    let signature = gen_signature(key, url, &params_str);

    params.push(("oauth_signature", signature));
}

fn generate_nonce() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(42).collect()
}

pub fn gen_auth_params(consumer_key: &str) -> Vec<(&'static str, String)> {
    let now: DateTime<Local> = std::time::SystemTime::now().into();
    vec![
        ("oauth_nonce", generate_nonce()),
        ("oauth_signature_method", "HMAC-SHA1".to_string()),
        ("oauth_timestamp", now.timestamp().to_string()),
        ("oauth_consumer_key", consumer_key.to_string()),
        ("oauth_version", "1.0".to_string()),
    ]
    .into_iter()
    .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HttpMethod {
    POST,
    GET,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::POST => write!(f, "POST"),
            Self::GET => write!(f, "GET"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    method: HttpMethod,
    base_url: String,
    queries: Vec<(&'static str, String)>, // parameters and queries should not be
    parameters: Vec<(&'static str, String)>, // URL encoded.
}

impl Request {
    pub fn get<S: ToString>(base_url: S) -> Self {
        Self {
            method: HttpMethod::GET,
            base_url: base_url.to_string(),
            queries: Vec::new(),
            parameters: Vec::new(),
        }
    }

    pub fn post<S: ToString>(base_url: S) -> Self {
        Self {
            method: HttpMethod::POST,
            base_url: base_url.to_string(),
            queries: Vec::new(),
            parameters: Vec::new(),
        }
    }

    pub fn query<V: ToString>(mut self, key: &'static str, value: V) -> Self {
        self.queries.push((key, value.to_string()));
        self
    }

    pub fn parameter<V: ToString>(mut self, key: &'static str, value: V) -> Self {
        self.parameters.push((key, value.to_string()));
        self
    }

    /// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
    pub fn create_signature(
        &self,
        consumer_key: &str,
        oauth_token: &str,
        consumer_secret: &str,
        oauth_token_secret: &str,
        oauth_nonce: &str,
        oauth_timestamp: i64,
    ) -> [u8; 20] {
        let signature_base_string =
            self.get_signature_base_string(consumer_key, oauth_token, oauth_nonce, oauth_timestamp);

        let signing_key = get_signing_key(consumer_secret, oauth_token_secret);

        hmacsha1::hmac_sha1(signing_key.as_bytes(), signature_base_string.as_bytes())
    }

    fn get_signature_base_string(
        &self,
        consumer_key: &str,
        oauth_token: &str,
        oauth_nonce: &str,
        oauth_timestamp: i64,
    ) -> String {
        let mut params = vec![
            (percent_encode("oauth_nonce"), percent_encode(oauth_nonce)),
            (
                percent_encode("oauth_signature_method"),
                percent_encode("HMAC-SHA1"),
            ),
            (
                percent_encode("oauth_timestamp"),
                percent_encode(&oauth_timestamp.to_string()),
            ),
            (
                percent_encode("oauth_consumer_key"),
                percent_encode(consumer_key),
            ),
            (percent_encode("oauth_version"), percent_encode("1.0")),
            (percent_encode("oauth_token"), percent_encode(oauth_token)),
        ];

        for (key, value) in &self.queries {
            params.push((percent_encode(key), percent_encode(value)));
        }

        for (key, value) in &self.parameters {
            params.push((percent_encode(key), percent_encode(value)));
        }

        params.sort();
        let param_string = params
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("&");

        format!(
            "{}&{}&{}",
            percent_encode(&self.method.to_string()),
            percent_encode(&self.base_url),
            percent_encode(&param_string)
        )
    }
}

fn get_signing_key(consumer_secret: &str, oauth_token_secret: &str) -> String {
    format!(
        "{}&{}",
        percent_encode(consumer_secret),
        percent_encode(oauth_token_secret)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encoding() {
        assert_eq!(
            percent_encode("Ladies + Gentlemen"),
            "Ladies%20%2B%20Gentlemen"
        );
        assert_eq!(
            percent_encode("An encoded string!"),
            "An%20encoded%20string%21"
        );
        assert_eq!(
            percent_encode("Dogs, Cats & Mice"),
            "Dogs%2C%20Cats%20%26%20Mice"
        );
        assert_eq!(percent_encode("☃"), "%E2%98%83");
    }

    #[test]
    fn signature_base_string() {
        let request = Request::post("https://api.twitter.com/1.1/statuses/update.json")
            .query("include_entities", "true")
            .parameter(
                "status",
                "Hello Ladies + Gentlemen, a signed OAuth request!",
            );

        let consumer_key = "xvz1evFS4wEEPTGEFPHBog";
        let oauth_token = "370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb";
        let oauth_nonce = "kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg";
        let oauth_timestamp = 1318622958;

        assert_eq!(
            request.get_signature_base_string(consumer_key, oauth_token, oauth_nonce, oauth_timestamp),
            "POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fupdate.json&include_entities%3Dtrue%26oauth_consumer_key%3Dxvz1evFS4wEEPTGEFPHBog%26oauth_nonce%3DkYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1318622958%26oauth_token%3D370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb%26oauth_version%3D1.0%26status%3DHello%2520Ladies%2520%252B%2520Gentlemen%252C%2520a%2520signed%2520OAuth%2520request%2521"
            );
    }

    #[test]
    fn signing_key() {
        let consumer_secret = "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw";
        let oauth_token_secret = "LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE";
        assert_eq!(
            get_signing_key(consumer_secret, oauth_token_secret),
            "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw&LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE"
        );
    }

    #[test]
    fn create_signature() {
        let request = Request::post("https://api.twitter.com/1.1/statuses/update.json")
            .query("include_entities", "true")
            .parameter(
                "status",
                "Hello Ladies + Gentlemen, a signed OAuth request!",
            );

        let consumer_key = "xvz1evFS4wEEPTGEFPHBog";
        let consumer_secret = "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw";
        let oauth_token = "370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb";
        let oauth_token_secret = "LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE";
        let oauth_nonce = "kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg";
        let oauth_timestamp = 1318622958;

        assert_eq!(
            request.create_signature(
                consumer_key,
                oauth_token,
                consumer_secret,
                oauth_token_secret,
                oauth_nonce,
                oauth_timestamp
            ),
            [
                0x84, 0x2B, 0x52, 0x99, 0x88, 0x7E, 0x88, 0x76, 0x02, 0x12, 0xA0, 0x56, 0xAC, 0x4E,
                0xC2, 0xEE, 0x16, 0x26, 0xB5, 0x49
            ]
        );
    }
}
