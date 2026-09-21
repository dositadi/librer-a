use jiff::{ Timestamp, civil::Date };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Debug, Clone, toasty::Model, Serialize, Deserialize)]
pub struct Book {
    #[key]
    #[auto]
    pub id: Uuid,

    pub published_date: Date,

    pub status: BookStatus,

    pub title: String,

    pub description: Option<String>,

    pub image_url: Option<String>,

    #[auto]
    pub created_at: Timestamp,

    #[auto]
    pub updated_at: Timestamp,
}

#[derive(Clone, Copy, PartialEq, Debug, toasty::Embed, Serialize, Deserialize)]
#[column(type = u8)]
#[serde(rename_all = "lowercase")]
pub enum BookStatus {
    #[column(variant = 0)]
    Pending,

    #[column(variant = 1)]
    Verified,
}
