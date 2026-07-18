use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

use crate::{AppState, auth::Claims, models::{AppError, DefineTable}, utils::utils::vacumm};

pub async fn delete_table(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(table_name): Path<String>
) -> Result<impl IntoResponse, AppError> {
    sqlx::query!(
        "DELETE FROM table_definitions WHERE tenant_id = $1 AND table_name = $2",
        claims.tenant_id,
        table_name,
    )
    .execute(&state.db)
    .await
    .map_err(AppError::Database)?;

    let cache_key = format!("{}:{}", claims.tenant_id, table_name);
    state.schemas.remove(cache_key, new_table_master);

    let _ = vacumm(&state.db, &state.schemas, &claims.tenant_id, &table_name).await;

    Ok(StatusCode::NO_CONTENT)
}