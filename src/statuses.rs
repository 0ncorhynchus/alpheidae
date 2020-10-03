use crate::error::*;
use crate::utils::*;
use crate::*;

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
