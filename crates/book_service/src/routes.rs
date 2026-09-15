use axum::{ Router, http::StatusCode, response::IntoResponse, routing::get };

pub fn init() -> Router {
    Router::new().route("/livez", get(livez))
}

async fn livez() -> impl IntoResponse {
    StatusCode::OK
}
