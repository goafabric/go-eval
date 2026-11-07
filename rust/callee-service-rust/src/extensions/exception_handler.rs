use std::fmt;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

#[derive(Debug)]
pub enum ServiceError {
    IllegalArgument(String),
    IllegalState(String),
    Internal(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::IllegalArgument(m)
            | ServiceError::IllegalState(m)
            | ServiceError::Internal(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for ServiceError {}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let status = match self {
            ServiceError::IllegalArgument(_) | ServiceError::IllegalState(_) => {
                StatusCode::PRECONDITION_FAILED
            }
            ServiceError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        error!("{self}");
        (status, format!("An error occured: {self}")).into_response()
    }
}
