use jiff::{ Timestamp, civil::Date };
use uuid::Uuid;

#[derive(Debug, Clone, toasty::Model)]
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

#[derive(Clone, Copy, PartialEq, Debug, toasty::Embed)]
#[column(type = u8)]
pub enum BookStatus {
    #[column(variant = 0)]
    Pending,

    #[column(variant = 1)]
    Verified,
}
