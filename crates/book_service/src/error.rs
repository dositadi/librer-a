use axum::{
    body::Body,
    http::{ HeaderValue, Response, StatusCode, header },
    response::IntoResponse,
};

pub enum APIError {
    ServerError,
    NotFound,
    RequestTimeout,
    Conflict,
    BadRequest,
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let (status, byte): (StatusCode, &[u8]) = match self {
            APIError::BadRequest => { (StatusCode::BAD_REQUEST, b"\"error\": \"invalid request\"") }
            APIError::Conflict => { (StatusCode::CONFLICT, b"\"error\": \"conflict\"") }
            APIError::NotFound => { (StatusCode::NOT_FOUND, b"\"error\": \"not found\"") }
            APIError::RequestTimeout => { (StatusCode::REQUEST_TIMEOUT, b"\"error\": \"timeout\"") }
            APIError::ServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, b"\"error\": \"something wrong happened\"")
            }
        };

        let mut response = Response::new(Body::from(byte));

        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

        *response.status_mut() = status;

        response
    }
}
