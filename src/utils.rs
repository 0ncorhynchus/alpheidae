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
    query: Vec<(&'static str, String)>, // parameters and queries should not be
    parameters: Vec<(&'static str, String)>, // URL encoded.
}

impl Request {
    /// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
    pub fn create_signature(
        &self,
        consumer_key: &str,
        oauth_token: &str,
        consumer_secret: &str,
        oauth_token_secret: &str,
    ) -> String {
        let mut params = Vec::new();
        for (key, value) in gen_auth_params(consumer_key) {
            params.push((percent_encode(key), percent_encode(&value)));
        }

        params.push((percent_encode("oauth_token"), percent_encode(oauth_token)));

        for (key, value) in &self.query {
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

        let signature_base_string = format!(
            "{}&{}&{}",
            percent_encode(&self.method.to_string()),
            percent_encode(&self.base_url),
            percent_encode(&param_string)
        );

        let signing_key = format!(
            "{}&{}",
            percent_encode(consumer_secret),
            percent_encode(oauth_token_secret)
        );

        base64::encode(&hmacsha1::hmac_sha1(
            signing_key.as_bytes(),
            signature_base_string.as_bytes(),
        ))
    }
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
}
