use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Img {
    alt: String,
    img: Vec<u8>,
    img_type: ImgType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImgType {
    #[serde(rename = "whiskyImage")]
    WhiskyImage,
    #[serde(rename = "profileImage")]
    ProfileImage,
}
