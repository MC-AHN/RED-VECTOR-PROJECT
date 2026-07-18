use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::to_value;
use std::{collections::{self, HashMap}, sync::Arc};

use crate::{
    AppState,
    auth::Claims,
    models::{AppError, ColumnRules, DefineTable, RuleMetadata, TableMaster},
};

pub async fn create_table(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(table_name): Path<String>,
    Json(payload): Json<DefineTable>,
) -> Result<impl IntoResponse, AppError> {
    let columns = payload
        .columns
        .ok_or_else(|| AppError::InvalidInput("Columns is required!".into()))?;

    let input_permissions = payload
        .permissions
        .unwrap_or_else(|| vec![1, 0, 0, 15]);
    
    if input_permissions.len() != 4 {
        return Err(AppError::InvalidInput("Permissions array must contain exactly 4 elements".into()));
    }

    let permissions_i16: Vec<i16> = input_permissions.iter().map(|&x| x as i16).collect();

    let mut current_id = 0;
    let mut input_columns = columns.clone();
    for col in &mut input_columns {
        current_id += 1;
        col.id = Some(current_id);
    }

    let mut tx = state.db.begin().await.map_err(AppError::Database)?;
    let current_tenant_id = claims.tenant_id;

    let schema_json = to_value(&input_columns)
        .map_err(|e| AppError::Internal(format!("Failed to serialize schema: {}", e)))?;

    sqlx::query!(
        r#"
            INSERT INTO table_definitions
            (tenant_id, table_name, schema_layout, permissions)
            VALUES ($1, $2, $3, $4)
            "#,
        current_tenant_id,
        table_name,
        schema_json,
        &permissions_i16
    )
    .execute(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    tx.commit().await.map_err(AppError::Database)?;

    let new_rules: Vec<ColumnRules> = input_columns
        .iter()
        .map(|col| ColumnRules {
            id: col.id.unwrap_or(0),
            name: col.column_name.clone(),
            rules: RuleMetadata {
                id: col.id.unwrap_or(0) as usize,
                data_type: col.data_type.clone(),
                is_required: col.is_required,
                min: col.min_val.map(|v| v as f64),
                max: col.max_val.map(|v| v as f64),
                references: col.references.clone(),
            },
        })
        .collect();

    let new_table_master = Arc::new(TableMaster {
        columns: new_rules,
        index_map: HashMap::new(),
        permissions: input_permissions,
    });

    let cache_key = format!("{}:{}", claims.tenant_id, table_name);
    state.schemas.insert(cache_key, new_table_master);

    Ok(StatusCode::CREATED)
}
