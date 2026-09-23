use axum::{ Json, http::{ HeaderValue, StatusCode, header }, response::IntoResponse };
use serde_json::json;

pub enum APIError {
    ServerError,
    NotFound,
    RequestTimeout,
    Conflict,
    BadRequest,
    NoRows,
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let (status, err): (StatusCode, &str) = match self {
            APIError::BadRequest => { (StatusCode::BAD_REQUEST, "invalid request") }
            APIError::Conflict => { (StatusCode::CONFLICT, "conflict") }
            APIError::NotFound => { (StatusCode::NOT_FOUND, "not found") }
            APIError::RequestTimeout => { (StatusCode::REQUEST_TIMEOUT, "timeout") }
            APIError::ServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "something wrong happened")
            }
            APIError::NoRows => { (StatusCode::NOT_FOUND, "no rows") }
        };

        let body = Json(
            json!({
                "error":err,
                "status": status.as_str(),
            })
        );

        let mut response = body.into_response();

        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

        *response.status_mut() = status;

        response
    }
}
