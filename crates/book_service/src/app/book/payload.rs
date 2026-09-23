use jiff::civil::Date;
use serde::{ Deserialize, Serialize };

use crate::models::BookStatus;

#[derive(Debug, Clone, Serialize, Deserialize, garde::Validate)]
pub struct BookRequest {
    #[garde(length(min = 1, max = 255))]
    pub title: String,

    #[garde(skip)]
    pub description: Option<String>,

    #[garde(url)]
    pub image_url: Option<String>,

    #[garde(skip)]
    pub published_date: Date,

    #[garde(skip)]
    pub status: BookStatus,
}
