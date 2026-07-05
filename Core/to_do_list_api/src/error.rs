use axum::response::{IntoResponse, Json, Response};

#[derive(Debug, Clone)]
pub enum Error {
    MutexLockFailed(&'static str),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::MutexLockFailed(message) => Json(serde_json::json!({ "error": format!("MutexLockFailed: {}", message) })).into_response()
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;