use jiff::civil::Date;
use serde::{ Deserialize, Serialize };

use crate::models::BookStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookRequest {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub published_date: Date,
    pub status: BookStatus,
}
