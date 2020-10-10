use crate::error::*;
use crate::utils::*;
use crate::*;
use serde::{Deserialize, Serialize};

pub fn upload_init(tokens: &TokenKeys, total_bytes: u32, media_type: String) -> UploadInitRequest {
    UploadInitRequest::new(tokens, total_bytes, media_type)
}

pub struct UploadInitRequest<'a> {
    tokens: &'a TokenKeys,
    total_bytes: u32,
    media_type: String,
    media_category: Option<String>,
    additional_owners: Vec<u64>,
}

impl<'a> UploadInitRequest<'a> {
    pub fn new(tokens: &'a TokenKeys, total_bytes: u32, media_type: String) -> Self {
        Self {
            tokens,
            total_bytes,
            media_type,
            media_category: None,
            additional_owners: Vec::new(),
        }
    }

    pub async fn send(self) -> Result<UploadInitResponse> {
        let url = "https://upload.twitter.com/1.1/media/upload.json";
        let mut request = Request::post(url);
        request.query("total_bytes", self.total_bytes);
        request.query("media_type", self.media_type);

        _opt_query!(self, request, media_category);
        _opt_query!(self, request, additional_owners[]);

        Ok(request.send(self.tokens).await?.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    image_type: String,
    w: u32,
    h: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadInitResponse {
    media_id: u64,
    medi_id_string: String,
    size: u32,
    expires_after_secs: u32,
    image: Image,
}
