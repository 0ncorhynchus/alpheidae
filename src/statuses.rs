use crate::error::*;
use crate::utils::*;
use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

pub fn update(tokens: &TokenKeys, status: String) -> Update {
    Update::new(tokens, status)
}

pub struct Update<'a> {
    tokens: &'a TokenKeys,
    status: String,
    in_reply_to_status_id: Option<u64>,
    auto_populate_reply_metadata: Option<bool>,
    exclude_reply_user_ids: Vec<u64>,
    attachment_url: Option<String>,
    media_ids: Vec<u64>, // 4 photo ids at most
    possibly_sensitive: Option<bool>,
    lat: Option<f64>,
    long: Option<f64>,
    place_id: Option<String>,
    display_coordinates: Option<bool>,
}

impl<'a> Update<'a> {
    pub fn new(tokens: &'a TokenKeys, status: String) -> Self {
        Self {
            tokens,
            status,
            in_reply_to_status_id: None,
            auto_populate_reply_metadata: None,
            exclude_reply_user_ids: Vec::new(),
            attachment_url: None,
            media_ids: Vec::new(),
            possibly_sensitive: None,
            lat: None,
            long: None,
            place_id: None,
            display_coordinates: None,
        }
    }

    pub async fn send(self) -> Result<Tweet> {
        let url = "https://api.twitter.com/1.1/statuses/update.json";
        let mut request = Request::post(url);
        request.query("status", self.status);

        macro_rules! opt_param {
            ($var:ident) => {
                if let Some(param) = self.$var {
                    request.parameter(stringify!($var), param);
                }
            };
            ($var:ident[]) => {
                if !self.$var.is_empty() {
                    request.parameter(
                        stringify!($var),
                        self.$var
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(","),
                    );
                }
            };
        }

        opt_param!(in_reply_to_status_id);
        opt_param!(auto_populate_reply_metadata);
        opt_param!(exclude_reply_user_ids[]);
        opt_param!(attachment_url);
        opt_param!(media_ids[]);
        opt_param!(possibly_sensitive);
        opt_param!(lat);
        opt_param!(long);
        opt_param!(place_id);
        opt_param!(display_coordinates);

        Ok(request.send(self.tokens).await?.json().await?)
    }
}

pub fn destroy(tokens: &TokenKeys, id: u64) -> Destroy {
    Destroy::new(tokens, id)
}

pub struct Destroy<'a> {
    tokens: &'a TokenKeys,
    id: u64,
    trim_user: Option<bool>,
}

impl<'a> Destroy<'a> {
    pub fn new(tokens: &'a TokenKeys, id: u64) -> Self {
        Self {
            tokens,
            id,
            trim_user: None,
        }
    }

    pub async fn send(self) -> Result<Tweet> {
        let url = format!(
            "https://api.twitter.com/1.1/statuses/destroy/{}.json",
            self.id
        );
        let mut request = Request::post(url);
        if let Some(trim_user) = self.trim_user {
            request.parameter("trim_user", trim_user);
        }
        Ok(request.send(self.tokens).await?.json().await?)
    }
}

pub fn show(tokens: &TokenKeys, id: u64) -> Show {
    Show::new(tokens, id)
}

pub struct Show<'a> {
    tokens: &'a TokenKeys,
    id: u64,
    trim_user: Option<bool>,
    include_my_retweet: Option<bool>,
    include_entities: Option<bool>,
    include_ext_alt_text: Option<bool>,
    include_card_uri: Option<bool>,
}

impl<'a> Show<'a> {
    pub fn new(tokens: &'a TokenKeys, id: u64) -> Self {
        Self {
            tokens,
            id,
            trim_user: None,
            include_my_retweet: None,
            include_entities: None,
            include_ext_alt_text: None,
            include_card_uri: None,
        }
    }

    pub async fn send(self) -> Result<Tweet> {
        let url = "https://api.twitter.com/1.1/statuses/show.json";
        let mut request = Request::get(url);
        request.query("id", self.id);

        macro_rules! opt_query {
            ($var:ident) => {
                if let Some(param) = self.$var {
                    request.query(stringify!($var), param);
                }
            };
        }

        opt_query!(trim_user);
        opt_query!(include_my_retweet);
        opt_query!(include_entities);
        opt_query!(include_ext_alt_text);
        opt_query!(include_card_uri);

        Ok(request.send(self.tokens).await?.json().await?)
    }
}

pub fn oembed(tokens: &TokenKeys, url: String) -> OEmbedRequest {
    OEmbedRequest::new(tokens, url)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Align {
    Left,
    Right,
    Center,
    None,
}

impl Display for Align {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
            Self::Center => write!(f, "center"),
            Self::None => write!(f, "none"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Light => write!(f, "light"),
            Self::Dark => write!(f, "dark"),
        }
    }
}

pub struct OEmbedRequest<'a> {
    tokens: &'a TokenKeys,
    url: String,
    maxwidth: Option<u16>,
    hide_media: Option<bool>,
    hide_thread: Option<bool>,
    omit_script: Option<bool>,
    align: Option<Align>,
    related: Option<String>,
    lang: Option<String>, // Language TODO
    theme: Option<Theme>,
    link_color: Option<String>,
    widget_type: Option<String>, // Video TODO
    dnt: Option<bool>,
}

impl<'a> OEmbedRequest<'a> {
    pub fn new(tokens: &'a TokenKeys, url: String) -> Self {
        Self {
            tokens,
            url,
            maxwidth: None,
            hide_media: None,
            hide_thread: None,
            omit_script: None,
            align: None,
            related: None,
            lang: None,
            theme: None,
            link_color: None,
            widget_type: None,
            dnt: None,
        }
    }

    pub async fn send(self) -> Result<OEmbed> {
        let url = "https://publish.twitter.com/oembed";
        let mut request = Request::get(url);
        request.query("url", self.url);

        macro_rules! opt_query {
            ($var:ident) => {
                if let Some(param) = self.$var {
                    request.query(stringify!($var), param);
                }
            };
        }

        opt_query!(maxwidth);
        opt_query!(hide_media);
        opt_query!(hide_thread);
        opt_query!(omit_script);
        opt_query!(align);
        opt_query!(related);
        opt_query!(lang);
        opt_query!(theme);
        opt_query!(link_color);
        opt_query!(widget_type);
        opt_query!(dnt);

        Ok(request.send(self.tokens).await?.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OEmbed {
    pub url: String,
    pub author_name: String,
    pub author_url: String,
    pub html: String,
    pub width: u16,
    pub height: Option<u16>,
    pub r#type: String,
    pub cache_age: String,
    pub provider_name: String,
    pub provider_url: String,
    pub version: String,
}
