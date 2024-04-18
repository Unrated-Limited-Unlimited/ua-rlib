use rocket::Responder;
use serde::{Deserialize, Serialize};

#[deprecated(since = "1.3.2", note = "Not used")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Img {
    alt: String,
    img: Vec<u8>,
    img_type: ImgType,
    id: String,
}

#[deprecated(since = "1.3.2", note = "Not used")]
#[derive(Debug, Serialize, Deserialize)]
pub enum ImgType {
    #[serde(rename = "whiskyImage")]
    WhiskyImage,
    #[serde(rename = "profileImage")]
    ProfileImage,
}

/// Image Response
///
/// Sets response status and content-type header to image/jpeg
#[derive(Responder)]
#[response(status = 200, content_type = "image/jpeg")]
pub struct ImgResponse(pub Vec<u8>);

/// Image wrapper used for storing it in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct ImgWrapper {
    pub img: Vec<u8>,
    pub iid: String,
}
