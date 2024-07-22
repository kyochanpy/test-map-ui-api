use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use super::{errors::AppError, validate::ValidateRequest};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(errors) => {
                let errors = serde_json::to_string(&errors).unwrap();
                (StatusCode::BAD_REQUEST, errors).into_response()
            }
            AppError::JsonRejection(rejection) => rejection.into_response(),
        }
        .into_response()
    }
}

#[async_trait]
impl<T, B> FromRequest<B> for ValidateRequest<T>
where
    T: DeserializeOwned + Validate + Send,      // Ensure T is Send
    B: http_body::Body + Send + Sync + 'static, // Ensure B is Send and 'static
    B::Data: Send + Sync,                       // Ensure the data chunks are Send
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
        let body = Json::<T>::from_request(req, state)
            .await
            .map_err(AppError::JsonRejection)?;
        body.validate().map_err(AppError::Validation)?;

        Ok(ValidateRequest { 0: body.0 })
    }
}
