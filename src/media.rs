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
        request.query("command", "INIT");
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

pub fn upload_append(
    tokens: &TokenKeys,
    media_id: u64,
    media: Vec<u8>,
    segment_index: u16,
) -> UploadAppendRequest {
    UploadAppendRequest::new(tokens, media_id, media, segment_index)
}

pub struct UploadAppendRequest<'a> {
    tokens: &'a TokenKeys,
    media_id: u64,
    media: Vec<u8>,
    segment_index: u16,
}

impl<'a> UploadAppendRequest<'a> {
    pub fn new(tokens: &'a TokenKeys, media_id: u64, media: Vec<u8>, segment_index: u16) -> Self {
        Self {
            tokens,
            media_id,
            media,
            segment_index,
        }
    }

    pub async fn send(self) -> Result<()> {
        let url = "https://upload.twitter.com/1.1/media/upload.json";
        let mut request = Request::post(url);
        request.query("command", "APPEND");
        request.query("media_id", self.media_id);
        // request.query("media", self.media);
        request.query(
            "media_data",
            base64::encode(String::from_utf8(self.media).unwrap()),
        );
        request.query("segment_index", self.segment_index);
        request.send(self.tokens).await?;
        Ok(())
    }
}

pub fn upload_finalize(tokens: &TokenKeys, media_id: u64) -> UploadFinalizeRequest {
    UploadFinalizeRequest::new(tokens, media_id)
}

pub struct UploadFinalizeRequest<'a> {
    tokens: &'a TokenKeys,
    media_id: u64,
}

impl<'a> UploadFinalizeRequest<'a> {
    pub fn new(tokens: &'a TokenKeys, media_id: u64) -> Self {
        Self { tokens, media_id }
    }

    pub async fn send(self) -> Result<UploadFinalizeResponse> {
        let url = "https://upload.twitter.com/1.1/media/upload.json";
        let mut request = Request::post(url);
        request.query("command", "FINALIZE");
        request.query("media_id", self.media_id);

        Ok(request.send(self.tokens).await?.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadFinalizeResponse {
    media_id: u64,
    media_id_string: String,
    size: u32,
    expires_after_secs: u32,
    // processing_info or video or image // TODO
}
