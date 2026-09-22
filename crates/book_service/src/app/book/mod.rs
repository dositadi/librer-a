use axum::{ Router, routing::get };

use crate::AppState;

pub mod payload;
pub mod handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list).post(handlers::create))
        .route("/{id}", get(handlers::read).put(handlers::update).delete(handlers::delete))
}
