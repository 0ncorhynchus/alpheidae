use crate::utils::*;
use crate::*;
use actix_web::client::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};
use serde::Deserialize;

#[derive(Clone, Copy)]
pub enum AccessType {
    Read,
    Write,
}

impl std::fmt::Display for AccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "read"),
            Self::Write => write!(f, "write"),
        }
    }
}

pub fn request_token(keys: &KeyPair, oauth_callback: String) -> RequestToken {
    RequestToken::new(keys, oauth_callback)
}

pub struct RequestToken<'a> {
    consumer_keys: &'a KeyPair,
    oauth_callback: String,
    x_auth_access_type: Option<AccessType>,
}

impl<'a> RequestToken<'a> {
    pub fn new(keys: &'a KeyPair, oauth_callback: String) -> RequestToken<'a> {
        Self {
            consumer_keys: keys,
            oauth_callback,
            x_auth_access_type: None,
        }
    }

    pub fn x_auth_access_type(mut self, access_type: AccessType) -> Self {
        self.x_auth_access_type = Some(access_type);
        self
    }

    pub async fn send(self) -> RequestTokenResponse {
        let url = "https://api.twitter.com/oauth/request_token";
        let mut oauth_params = gen_auth_params(&self.consumer_keys.key);
        oauth_params.push(("oauth_callback", self.oauth_callback));
        if let Some(access_type) = self.x_auth_access_type {
            oauth_params.push(("x_auth_access_type", access_type.to_string()));
        }
        let signature_key = gen_signature_key(&self.consumer_keys.secret, "");
        write_signature(signature_key, url, &mut oauth_params, &[]);

        let builder = SslConnector::builder(SslMethod::tls()).unwrap();
        let client = Client::builder()
            .connector(Connector::new().ssl(builder.build()).finish())
            .finish();

        let mut res = client.post(url).oauth(oauth_params).send().await.unwrap();
        serde_qs::from_bytes(res.body().await.unwrap().as_ref()).unwrap()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RequestTokenResponse {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_callback_confirmed: bool,
}
