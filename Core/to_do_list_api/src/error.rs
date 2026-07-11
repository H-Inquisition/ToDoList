use axum::response::{IntoResponse, Json, Response};

#[derive(Debug, Clone)]
pub enum Error {
    MutexLockFailed(&'static str),
    DatabaseError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::MutexLockFailed(message) => {
                Json(serde_json::json!({ "error": format!("MutexLockFailed: {}", message) }))
                    .into_response()
            }
            Error::DatabaseError(message) => {
                Json(serde_json::json!({ "error": format!("DatabaseError: {}", message) }))
                    .into_response()
            }
        }
    }
}

impl From<core::error::Error> for Error {
    fn from(error: core::error::Error) -> Self {
        Error::DatabaseError(format!("{:?}", error))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
