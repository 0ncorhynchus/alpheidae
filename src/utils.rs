use crate::*;
use actix_web::client::{Client, Connector};
use actix_web::http::header::AUTHORIZATION;
use chrono::{offset::Local, DateTime};
use openssl::ssl::{SslConnector, SslMethod};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt;

pub fn percent_encode(input: &str) -> String {
    const FRAGMENTS: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~');
    utf8_percent_encode(input, FRAGMENTS).to_string()
}

fn generate_nonce() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(42).collect()
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
    oauth_params: Vec<(&'static str, String)>,
}

impl Request {
    pub fn get<S: ToString>(base_url: S) -> Self {
        Self {
            method: HttpMethod::GET,
            base_url: base_url.to_string(),
            queries: Vec::new(),
            parameters: Vec::new(),
            oauth_params: Vec::new(),
        }
    }

    pub fn post<S: ToString>(base_url: S) -> Self {
        Self {
            method: HttpMethod::POST,
            base_url: base_url.to_string(),
            queries: Vec::new(),
            parameters: Vec::new(),
            oauth_params: Vec::new(),
        }
    }

    pub fn query<V: ToString>(&mut self, key: &'static str, value: V) -> &mut Self {
        self.queries.push((key, value.to_string()));
        self
    }

    pub fn parameter<V: ToString>(&mut self, key: &'static str, value: V) -> &mut Self {
        self.parameters.push((key, value.to_string()));
        self
    }

    pub fn oauth_param<V: ToString>(&mut self, key: &'static str, value: V) -> &mut Self {
        self.oauth_params.push((key, value.to_string()));
        self
    }

    fn get_url(&self) -> String {
        if self.queries.is_empty() {
            self.base_url.clone()
        } else {
            let queries = self
                .queries
                .iter()
                .map(|(key, value)| format!("{}={}", percent_encode(key), percent_encode(value)))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", self.base_url, queries)
        }
    }

    pub fn send(&self, tokens: &TokenKeys) -> awc::SendClientRequest {
        let oauth_nonce = generate_nonce();
        let now: DateTime<Local> = std::time::SystemTime::now().into();

        let mut params = get_oauth_params(tokens, &oauth_nonce, now.timestamp());
        for (key, value) in &self.oauth_params {
            params.push((percent_encode(key), percent_encode(value)));
        }
        let signature = base64::encode(create_signature(tokens, self, params.clone()));
        let authorization_header = get_authorization_header(params, &signature);

        let builder = SslConnector::builder(SslMethod::tls()).unwrap();
        let client = Client::builder()
            .connector(Connector::new().ssl(builder.build()).finish())
            .finish();

        client
            .post(self.get_url())
            .header(AUTHORIZATION, authorization_header)
            .send()
    }
}

/// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature
fn create_signature(
    tokens: &TokenKeys,
    request: &Request,
    oauth_params: Vec<(String, String)>,
) -> [u8; 20] {
    let signature_base_string = get_signature_base_string(request, oauth_params);
    let signing_key = get_signing_key(tokens);
    hmacsha1::hmac_sha1(signing_key.as_bytes(), signature_base_string.as_bytes())
}

fn get_oauth_params(
    tokens: &TokenKeys,
    oauth_nonce: &str,
    oauth_timestamp: i64,
) -> Vec<(String, String)> {
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
            percent_encode(&tokens.consumer_keys.key),
        ),
        (percent_encode("oauth_version"), percent_encode("1.0")),
    ];

    if let Some(oauth_tokens) = &tokens.oauth_tokens {
        params.push((
            percent_encode("oauth_token"),
            percent_encode(&oauth_tokens.key),
        ));
    }

    params
}

fn get_signature_base_string(request: &Request, mut oauth_params: Vec<(String, String)>) -> String {
    for (key, value) in &request.queries {
        oauth_params.push((percent_encode(key), percent_encode(value)));
    }

    for (key, value) in &request.parameters {
        oauth_params.push((percent_encode(key), percent_encode(value)));
    }

    oauth_params.sort();
    let param_string = oauth_params
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&");

    format!(
        "{}&{}&{}",
        percent_encode(&request.method.to_string()),
        percent_encode(&request.base_url),
        percent_encode(&param_string)
    )
}

fn get_signing_key(tokens: &TokenKeys) -> String {
    let oauth_token_secret = tokens
        .oauth_tokens
        .as_ref()
        .map(|key_pair| percent_encode(&key_pair.secret))
        .unwrap_or("".to_string());
    format!(
        "{}&{}",
        percent_encode(&tokens.consumer_keys.secret),
        oauth_token_secret,
    )
}

fn get_authorization_header(mut params: Vec<(String, String)>, signature: &str) -> String {
    params.push((percent_encode("oauth_signature"), percent_encode(signature)));
    params.sort();
    format!(
        "OAuth {}",
        params
            .into_iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect::<Vec<_>>()
            .join(", ")
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

    fn get_tokens() -> TokenKeys {
        TokenKeys {
            consumer_keys: KeyPair::new(
                "xvz1evFS4wEEPTGEFPHBog".to_string(),
                "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw".to_string(),
            ),
            oauth_tokens: Some(KeyPair::new(
                "370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb".to_string(),
                "LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE".to_string(),
            )),
        }
    }

    const OAUTH_NONCE: &'static str = "kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg";
    const OAUTH_TIMESTAMP: i64 = 1318622958;

    #[test]
    fn signature_base_string() {
        let mut request = Request::post("https://api.twitter.com/1.1/statuses/update.json");
        request.query("include_entities", "true").parameter(
            "status",
            "Hello Ladies + Gentlemen, a signed OAuth request!",
        );
        let oauth_params = get_oauth_params(&get_tokens(), OAUTH_NONCE, OAUTH_TIMESTAMP);

        assert_eq!(
            get_signature_base_string(&request, oauth_params),
            "POST&https%3A%2F%2Fapi.twitter.com%2F1.1%2Fstatuses%2Fupdate.json&include_entities%3Dtrue%26oauth_consumer_key%3Dxvz1evFS4wEEPTGEFPHBog%26oauth_nonce%3DkYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1318622958%26oauth_token%3D370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb%26oauth_version%3D1.0%26status%3DHello%2520Ladies%2520%252B%2520Gentlemen%252C%2520a%2520signed%2520OAuth%2520request%2521"
            );
    }

    #[test]
    fn signing_key() {
        assert_eq!(
            get_signing_key(&get_tokens()),
            "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw&LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE"
        );
    }

    #[test]
    fn signature() {
        let mut request = Request::post("https://api.twitter.com/1.1/statuses/update.json");
        request.query("include_entities", "true").parameter(
            "status",
            "Hello Ladies + Gentlemen, a signed OAuth request!",
        );
        let oauth_params = get_oauth_params(&get_tokens(), OAUTH_NONCE, OAUTH_TIMESTAMP);

        assert_eq!(
            create_signature(&get_tokens(), &request, oauth_params),
            [
                0x84, 0x2B, 0x52, 0x99, 0x88, 0x7E, 0x88, 0x76, 0x02, 0x12, 0xA0, 0x56, 0xAC, 0x4E,
                0xC2, 0xEE, 0x16, 0x26, 0xB5, 0x49
            ]
        );
    }

    #[test]
    fn oauth_header() {
        let params = get_oauth_params(&get_tokens(), OAUTH_NONCE, OAUTH_TIMESTAMP);
        let signature = "tnnArxj06cWHq44gCs1OSKk/jLY=";
        assert_eq!(
            get_authorization_header(params, signature),
            r#"OAuth oauth_consumer_key="xvz1evFS4wEEPTGEFPHBog", oauth_nonce="kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg", oauth_signature="tnnArxj06cWHq44gCs1OSKk%2FjLY%3D", oauth_signature_method="HMAC-SHA1", oauth_timestamp="1318622958", oauth_token="370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb", oauth_version="1.0""#
        );
    }
}
