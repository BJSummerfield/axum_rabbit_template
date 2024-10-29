use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;
pub enum Error {
    ValidationError(String),
    NotFound(String),
    SerdeJson(serde_json::Error),
    Lapin(lapin::Error),
    Postgres(tokio_postgres::Error),
    Bb8(bb8::RunError<tokio_postgres::Error>),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            Error::NotFound(msg) => write!(f, "Not Found: {}", msg),
            Error::SerdeJson(err) => write!(f, "Serde JSON Error: {}", err),
            Error::Lapin(err) => write!(f, "Lapin Error: {}", err),
            Error::Postgres(err) => write!(f, "Postgres Error: {}", err),
            Error::Bb8(err) => write!(f, "BB8 Connection Pool Error: {}", err),
            Error::Other(err) => write!(f, "{}", err),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson(err)
    }
}

impl From<lapin::Error> for Error {
    fn from(err: lapin::Error) -> Self {
        Error::Lapin(err)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Error::Postgres(err)
    }
}

impl From<bb8::RunError<tokio_postgres::Error>> for Error {
    fn from(err: bb8::RunError<tokio_postgres::Error>) -> Self {
        Error::Bb8(err)
    }
}

// Fallback for any other error
impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::Other(err)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Implement IntoResponse for the Error enum
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Error::SerdeJson(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Serde JSON Error: {}", err),
            ),
            Error::Lapin(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Lapin Error: {}", err),
            ),
            Error::Postgres(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Postgres Error: {}", err),
            ),
            Error::Bb8(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("BB8 Connection Pool Error: {}", err),
            ),
            Error::Other(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)),
        };

        // Create the error response body
        let response = ErrorResponse {
            error: error_message,
        };

        // Convert to an Axum response with the determined status code and error message
        (status, Json(response)).into_response()
    }
}
