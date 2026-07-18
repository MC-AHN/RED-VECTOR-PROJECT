use dashmap::DashMap;
use sqlx::{PgPool, query};

use crate::TenantContext;

pub async fn initialize(pool: &PgPool) -> DashMap<String, TenantContext> {
    let token_cache = DashMap::new();

    let rows = query!(
        r#"
        SELECT id, account_id, tenant_name_slug, client_token
        FROM tenants
        WHERE client_token IS NOT NULL
        "#
    )
    .fetch_all(pool)
    .await
    .expect("Failed to load data token from database");

    for row in rows {
        token_cache.insert(
            row.client_token,
            TenantContext {
                tenant_id: row.id,
                account_id: row.account_id,
                tenant_name_slug: row.tenant_name_slug,
            },
        );
    }

    println!(" RAM Cache Server B: {} succeseed to load", token_cache.len());
    token_cache
}