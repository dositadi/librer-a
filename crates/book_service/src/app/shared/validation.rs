use std::{ borrow::Cow, collections::HashMap };

use axum::{
    Json,
    extract::{ FromRequest, rejection::JsonRejection },
    http::StatusCode,
    response::IntoResponse,
};
use garde::{ I18n, Report, Validate, i18n, with_i18n };
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
        with_i18n(English, || value.validate())?;
        Ok(ValidatedJSON(value))
    }
}

struct English;

impl I18n for English {
    fn alphanumeric_invalid(&self) -> std::borrow::Cow<'static, str> {
        Cow::Borrowed("Must contain only letters and numbers")
    }

    fn ascii_invalid(&self) -> Cow<'static, str> {
        Cow::Borrowed("Must contain only ASCII Characters")
    }

    fn contains_missing(&self, pattern: &dyn std::fmt::Display) -> Cow<'static, str> {
        format!("Must contain \"{pattern}\"").into()
    }

    fn credit_card_invalid(&self, reason: i18n::InvalidCreditCard) -> Cow<'static, str> {
        format!("{}: Must be a valid Credit card number", reason).into()
    }

    fn email_invalid(&self, reason: i18n::InvalidEmail) -> Cow<'static, str> {
        format!("{}: Must be a valid email", reason).into()
    }

    fn ip_invalid(&self, kind: i18n::IpKind) -> Cow<'static, str> {
        format!("Must be a valid {kind} IP address").into()
    }

    fn length_greater_than(&self, max: usize) -> Cow<'static, str> {
        match max {
            1 => "Length must not exceed 1 character".to_string().into(),
            _ => format!("Length must not exceed {max} character").into(),
        }
    }

    fn length_lower_than(&self, min: usize) -> Cow<'static, str> {
        match min {
            1 => "Length must exceed 1 character".to_string().into(),
            _ => format!("Length must exceed {min} character").into(),
        }
    }

    fn matches_field_mismatch(&self, field: &dyn std::fmt::Display) -> Cow<'static, str> {
        format!("Must match the {field} field").into()
    }

    fn pattern_no_match(&self, pattern: &dyn std::fmt::Display) -> Cow<'static, str> {
        format!("Must match the required format: \"{pattern}\"").into()
    }

    fn phone_number_invalid(&self, reason: i18n::InvalidPhoneNumber) -> Cow<'static, str> {
        format!("{reason}: Must be a valid phone number").into()
    }

    fn prefix_missing(&self, pattern: &dyn std::fmt::Display) -> Cow<'static, str> {
        format!("Must start with \"{pattern}\"").into()
    }

    fn range_greater_than(&self, max: &dyn std::fmt::Display) -> Cow<'static, str> {
        format!("Must be less than or equal to {max}").into()
    }

    fn range_lower_than(&self, min: &dyn std::fmt::Display) -> Cow<'static, str> {
        format!("Must be greater than or equal to {min}").into()
    }

    fn required_not_set(&self) -> Cow<'static, str> {
        Cow::Borrowed("This field is required")
    }

    fn suffix_missing(&self, pattern: &dyn std::fmt::Display) -> Cow<'static, str> {
        format!("Must end with \"{pattern}\"").into()
    }

    fn url_invalid(&self, reason: i18n::InvalidUrl) -> Cow<'static, str> {
        format!("{reason}: Must be a valid url").into()
    }
}
