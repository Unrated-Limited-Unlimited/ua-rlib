use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Img {
    alt: String,
    img: Vec<u8>,
}
