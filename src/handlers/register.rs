use std::sync::Arc;
use axum::{extract::State, Json, http::StatusCode};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::query;
use uuid::Uuid;

use crate::{AppState, TenantContext, models::{auth::{RegisterPayload, RegisterResponse}, error::AppError}};



fn generate_tokern() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    format!("rv_live_{}", random_string)
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>
) -> Result<(StatusCode, Json<RegisterResponse>), AppError> {
    let hashed_password = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::Internal("Failed to hash Password".into()))?;
    
    let mut tx = state.db.begin().await.map_err(AppError::Database)?;

    let account_row = query!(
        r#"
        INSERT INTO accounts (email, password_hash)
        VALUES ($1, $2)
        RETURNING id
        "#,
        paylaod.email,
        hashed_password
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    let new_account_id = account_row.id;
    let token = generate_token();

    let tenant_row = query!(
        r#"
        INSERT INTO tenants (account_id, tenant_name_slug, client_token)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        new_account_id,
        payload.tenant_name_slug,
        token
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    let new_tenant_id = tenant_row.id;

    tx.commit().await.map_err(AppError::Database)?;

    state.token_cache.insert(
        token.clone(),
        TenantContext {
            tenant_id: new_tenant_id,
            account_id: new_account_id,
            tenant_name_slug: payload.tenant_name_slug.clone(),
        },
    );

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            account_id: new_account_id,
            tenant_id: new_tenant_id,
            tenant_name_slug: payload.tenant_name_slug,
            client_token: token
        })
    ))
}