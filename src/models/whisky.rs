use super::raw_whisky::RawWhisky;
use eyre::{Context, OptionExt, Result};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Whiskey {
    pub id: u32,
    pub img: String,
    pub percentage: u32,
    pub price: f32,
    pub rating: f32,
    pub summary: String,
    pub title: String,
    pub volume: f32,
}

impl Whiskey {
    pub fn new(
        id: u32,
        img: String,
        percentage: u32,
        price: f32,
        rating: f32,
        summary: String,
        title: String,
        volume: f32,
    ) -> Self {
        Whiskey {
            id,
            img,
            percentage,
            price,
            rating,
            summary,
            title,
            volume,
        }
    }

    pub fn from_raw(mut raw: RawWhisky) -> Self {
        Whiskey {
            id: 0,
            img: raw.images.pop().expect("Expected an image").url,
            percentage: raw.alcohol.value as u32,
            price: raw.price.value,
            rating: 0f32,
            summary: raw.summary,
            title: raw.name,
            volume: raw.volume.value,
        }
    }

    pub fn from_value(value: Value) -> Result<Self> {
        Ok(Whiskey {
            id: 0,
            img: value["images"]
                .as_array()
                .ok_or_eyre(format!("Failed getting `images` from `{value:?}`"))?[0]["url"]
                .to_string(),
            percentage: value["alcohol"]["value"]
                .to_string()
                .parse::<u32>()
                .wrap_err(format!("Failed getting `alcohol.value` from `{value:?}`"))?,
            price: value["price"]["value"]
                .to_string()
                .parse::<f32>()
                .wrap_err(format!("Failed getting `price.value` from `{value:?}`"))?,
            rating: 0f32,
            summary: value["summary"].to_string(),
            title: value["name"].to_string(),
            volume: value["volume"]["value"]
                .to_string()
                .parse::<f32>()
                .wrap_err(format!("Failed getting `price.value` from `{value:?}`"))?,
        })
    }
}
