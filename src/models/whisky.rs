use serde::{Deserialize, Serialize};

use super::raw_whisky::RawWhisky;

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
