use axum::{Json, extract::State};
use serde::Deserialize;
use sqlx::query;
use serde_json::{json, Value};

use crate::{AppState, models::error::AppError};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password_plain: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<Value>, AppError> {
    let user = query!(
        "SELECT id, password FROM accounts WHERE email = $1 LIMIT 1",
        paylaod.email
    ).fetch_optional(&state.db)
    .await
    .map_err(AppError::DATABASE)
    .ok_or_else(|| AppError::Unauthorized("Email or Password Invalid.".into()))?;

    if user.password_hash != payload.password_plain {
        return Err(AppError::Unauthorized("Email or Password Invalid".into()));
    }

    let tenant = query!(
        "SELECT id from tenants WHERE account_id = $1 LIMIT 1",
        user.id
    )
    .fetch_one(&state.db)
    .await
    .map_err(AppError::Database)?;

    let token = generate_jwt(user.id, tenant.id, 1)?;

    Ok(Json(json!({
        "Status": "Success",
        "token": token
    })))
}