use crate::error::*;
use crate::utils::*;
use crate::*;
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

    pub async fn send(self) -> Result<RequestTokenResponse> {
        let url = "https://api.twitter.com/oauth/request_token";
        let tokens = TokenKeys::new(self.consumer_keys.clone());
        let mut request = Request::post(url);
        request.oauth_param("oauth_callback", &self.oauth_callback);
        _opt_query!(self, request, x_auth_access_type);
        let mut res = request.send(&tokens).await?;
        let body = res.body().await?;
        Ok(serde_qs::from_bytes(body.as_ref())?)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct RequestTokenResponse {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_callback_confirmed: bool,
}

impl RequestTokenResponse {
    pub fn authorize(&self) -> Authorize {
        Authorize::new(self.oauth_token.clone())
    }

    pub fn get_redirect_url(&self) -> String {
        self.authorize().into_url()
    }
}

pub struct Authorize {
    oauth_token: String,
    force_login: Option<bool>,
    screen_name: Option<String>,
}

impl Authorize {
    pub fn new(oauth_token: String) -> Self {
        Self {
            oauth_token,
            force_login: None,
            screen_name: None,
        }
    }

    pub fn force_login(mut self, force_login: bool) -> Self {
        self.force_login = Some(force_login);
        self
    }

    pub fn screen_name(mut self, screen_name: String) -> Self {
        self.screen_name = Some(screen_name);
        self
    }

    pub fn into_url(self) -> String {
        let mut url = format!(
            "https://api.twitter.com/oauth/authorize?oauth_token={}",
            self.oauth_token
        );
        if let Some(force_login) = self.force_login {
            url += &format!("&force_login={}", force_login);
        }
        if let Some(screen_name) = self.screen_name {
            url += &format!("&screen_name={}", percent_encode(&screen_name));
        }
        url
    }
}

pub fn access_token(keys: &KeyPair, oauth_token: String, oauth_verifier: String) -> AccessToken {
    AccessToken::new(keys, oauth_token, oauth_verifier)
}

pub struct AccessToken<'a> {
    consumer_keys: &'a KeyPair,
    oauth_token: String,
    oauth_verifier: String,
}

impl<'a> AccessToken<'a> {
    pub fn new(consumer_keys: &'a KeyPair, oauth_token: String, oauth_verifier: String) -> Self {
        Self {
            consumer_keys,
            oauth_token,
            oauth_verifier,
        }
    }

    pub async fn send(self) -> Result<AccessTokenResponse> {
        let url = "https://api.twitter.com/oauth/access_token";
        let tokens = TokenKeys::new(self.consumer_keys.clone());
        let mut res = Request::post(url)
            .oauth_param("oauth_token", &self.oauth_token)
            .oauth_param("oauth_verifier", &self.oauth_verifier)
            .send(&tokens)
            .await?;
        let body = res.body().await?;
        Ok(serde_qs::from_bytes(body.as_ref())?)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct AccessTokenResponse {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub user_id: u64,
    pub screen_name: String,
}

pub fn invalidate_token(tokens: &TokenKeys) -> InvalidateToken {
    InvalidateToken::new(tokens)
}

pub struct InvalidateToken<'a> {
    tokens: &'a TokenKeys,
}

impl<'a> InvalidateToken<'a> {
    pub fn new(tokens: &'a TokenKeys) -> Self {
        Self { tokens }
    }

    pub async fn send(self) -> Result<()> {
        let url = "https://api.twitter.com/1.1/oauth/invalidate_token";
        let _res = Request::post(url).send(self.tokens).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorize_url() {
        let response = RequestTokenResponse {
            oauth_token: "Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hhlSLik".to_string(),
            oauth_token_secret: "".to_string(),
            oauth_callback_confirmed: true,
        };
        assert_eq!(response.get_redirect_url(), "https://api.twitter.com/oauth/authorize?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hhlSLik");
    }
}
