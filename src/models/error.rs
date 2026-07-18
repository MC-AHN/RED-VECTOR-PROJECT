use axum::{Json, http::StatusCode, response::{Response, IntoResponse}};
use serde::Serialize;
use sqlx::{Error, Error::{Database, RowNotFound}};

pub fn map_db_error(e: Error) -> (StatusCode, String) {
    match e {
        RowNotFound => (StatusCode::NOT_FOUND, "Record not found".into()),
        Database(db_err) => {
            // cek kalau tablenya nggak ada
            if db_err.code().as_deref() == Some("42P01") {
                (
                    StatusCode::BAD_REQUEST,
                    "Table does not exist or is not registered".into(),
                )
            } else if db_err.code().as_deref() == Some("23505") {
                (
                    StatusCode::CONFLICT,
                    "Duplicate entry: Data already exists".into(),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database Error: {}", db_err.message()),
                )
            }
        }
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unexpected system error occurred".into(),
        ),
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    // T is for generic automations, so if we enter any data the machine can generic type data secara otomatis
    pub status: String,
    pub message: String,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
}

#[derive(Debug, Serialize)]
pub enum AppError {
    #[serde(skip_serializing)]
    Database(Error),
    InvalidInput(String),
    NotFound(String),
    Unauthorized(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => map_db_error(e),
            AppError::InvalidInput(m) => (StatusCode::BAD_REQUEST, m),
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };

        let body = Json(ApiResponse::<()> {
            status: "Error".into(),
            message,
            data: None,
            count: None,
        });

        (status, body).into_response()
    }
}
