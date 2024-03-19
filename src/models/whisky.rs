use super::raw_whisky::RawWhisky;
use eyre::{Context, OptionExt, Result};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Whiskey {
    pub id: u64,
    pub img: String,
    pub percentage: u64,
    pub price: f64,
    pub rating: f64,
    pub summary: String,
    pub title: String,
    pub volume: f64,
}

impl Whiskey {
    pub fn new(
        id: u64,
        img: String,
        percentage: u64,
        price: f64,
        rating: f64,
        summary: String,
        title: String,
        volume: f64,
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
            percentage: raw.alcohol.value as u64,
            price: raw.price.value as f64,
            rating: 0f64,
            summary: raw.summary,
            title: raw.name,
            volume: raw.volume.value as f64,
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
                .as_number()
                .and_then(|e| e.as_f64())
                .ok_or_eyre(format!("Failed getting `alcohol.value` from `{value:?}`"))?
                as u64,
            price: value["price"]["value"]
                .as_number()
                .and_then(|e| e.as_f64())
                .ok_or_eyre(format!("Failed getting `price.value` from `{value:?}`"))?,
            rating: 0f64,
            summary: value["summary"].to_string(),
            title: value["name"].to_string(),
            volume: value["volume"]["value"]
                .as_number()
                .and_then(|e| e.as_f64())
                .ok_or_eyre(format!("Failed getting `price.value` from `{value:?}`"))?,
        })
    }
}
