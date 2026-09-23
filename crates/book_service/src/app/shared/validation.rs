use std::collections::HashMap;

use axum::{
    Json,
    extract::{ FromRequest, rejection::JsonRejection },
    http::StatusCode,
    response::IntoResponse,
};
use garde::{ Report, Validate };
use serde::{ Serialize, de::DeserializeOwned };

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub errors: HashMap<String, String>,
}

#[derive(Debug)]
pub enum ServerError {
    ValidationError(Report),
    AxumJsonRejection(JsonRejection),
}

impl From<Report> for ServerError {
    fn from(value: Report) -> Self {
        Self::ValidationError(value)
    }
}

impl From<JsonRejection> for ServerError {
    fn from(value: JsonRejection) -> Self {
        Self::AxumJsonRejection(value)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::AxumJsonRejection(rejection) => {
                (StatusCode::BAD_REQUEST, rejection).into_response()
            }
            Self::ValidationError(report) => {
                let errors = report
                    .iter()
                    .map(|(path, error)| (path.to_string(), error.message().to_string()))
                    .collect::<HashMap<_, _>>();

                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ValidationErrorResponse { errors }),
                ).into_response()
            }
        }
    }
}

#[derive(Clone, Debug, Copy, Default)]
pub struct ValidatedJSON<T>(pub T);

impl<S, T> FromRequest<S>
    for ValidatedJSON<T>
    where
        T: DeserializeOwned + Validate<Context = ()>,
        S: Send + Sync,
        Json<T>: FromRequest<S, Rejection = JsonRejection>
{
    type Rejection = ServerError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJSON(value))
    }
}
