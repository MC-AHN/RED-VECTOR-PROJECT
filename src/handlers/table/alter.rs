use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::{Value, to_value};
use sqlx::query;

use crate::{
    AppState,
    auth::Claims,
    models::{Action, AppError, ColumnRules, ColumnType, DefineTable, RuleMetadata, TableMaster}, utils::utils::vacumm,
};

pub async fn update_table(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(table_name): Path<String>,
    Json(payload): Json<DefineTable>,
) -> Result<impl IntoResponse, AppError> {
    let mut input_columns = payload.columns.clone().unwrap_or_default();

    let cache_key = format!("{}:{}", claims.tenant_id, table_name);

    {
        let table_info = state.schemas.get(&table_name).ok_or_else(|| {
            AppError::NotFound(format!("Table '{}' not found", table_name))
        })?;

        let mut max_id = table_info.columns.iter().map(|c| c.id).max().unwrap_or(0);
        for col in &mut input_columns {
            if let Some(rule) = table_info
                .columns
                .iter()
                .find(|r| r.name == col.column_name)
            {
                col.id = Some(rule.id);

                if rule.rules.data_type.to_lowercase() != col.data_type.to_lowercase() {
                    return Err(AppError::InvalidInput(format!(
                        "Failed! Type Data column '{}' can't change.",
                        col.column_name
                    )));
                }
            } else {
                max_id += 1;
                col.id = Some(max_id);
            }
        }
    }

    let update_schema = to_value(&input_columns)
        .map_err(|e| AppError::Internal(format!("Failed to serialize schema: {}", e)))?;

    query!(
        r#"
        UPDATE table_definitions
        SET schema_layout = $1
        WHERE tenant_id = $2 AND table_name = $3
        "#,
        update_schema,
        claims.tenant_id,
        table_name
    )
    .execute(&state.db)
    .await
    .map_err(AppError::Database)?;

    if let Some(table_ref) = state.schemas.get_mut(&cache_key) {
        let old_table = table_ref.value();

        let update_rules: Vec<ColumnRules> = input_columns
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
            columns: update_rules,
            index_map: old_table.index_map.clone(),
            permissions: old_table.permissions.clone(),
        });

        drop(table_ref);

        state
            .schemas
            .insert(cache_key.clone(), new_table_master);
    }

    let _ = vacumm(&state.db, &state.schemas, &claims.tenant_id, &table_name).await;

    Ok(StatusCode::OK)
}
