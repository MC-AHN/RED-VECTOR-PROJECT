use std::sync::Arc;

use axum::{Json, extract::{Path, State}, response::IntoResponse};
use serde_json::Value;
use sqlx::PgPool;
use serde::Serialize;

use crate::{AppState, auth::Claims, models::{AppError, DefineTable, ColumnDefinition}};

#[derive(Serialize)] 
pub struct TableNameOnly{
    pub table_name: String,
}

#[derive(Serialize)]
pub struct TableDetailResponse {
    pub table_name: String,
    pub schema_layout: Value
}

pub async fn read_all_tables(
    State(state): State<Arc<AppState>>,
    claims: Claims
) -> Result<impl IntoResponse, AppError> {
    let tables = sqlx::query_as!(
        TableNameOnly,
        r#"
        SELECT DISTINCT table_name FROM table_definitions WHERE tenant_id = $1
        "#,
        claims.tenant_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(AppError::Database)?;

    Ok(Json(tables))
}

pub async fn read_table_detail(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(table_name): Path<String>
) -> Result<impl IntoResponse, AppError> {
    let details = sqlx::query_as!(
        TableDetailResponse,
        r#"
        SELECT table_name, schema_layout 
        FROM table_definitions
        WHERE tenant_id = $1 AND table_name = $2
        LIMIT 1
        "#,
        claims.tenant_id,
        table_name,
    )
    .fetch_optional(&state.db) // use fetch_optional for sefty if table not found
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound(format!("Table '{}' not found", table_name)))?;

    Ok(Json(details))
}