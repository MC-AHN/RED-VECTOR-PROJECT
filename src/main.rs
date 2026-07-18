mod utils;
mod models;
mod handlers;
mod auth;

use std::{env, sync::Arc};

use axum::{Router, middleware::{FromFn, from_fn_with_state}, routing::{delete, get, post, put}, serve};
use dashmap::DashMap;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::{handlers::{login::login, register::register, table::{create_table, delete_table, read_all_tables, read_table_detail, update_table}}, utils::cache::initialize};

#[derive(Clone)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub account_id: Uuid,
    pub tenant_name_slug: String,
}

pub struct AppState {
    pub db: PgPool,
    pub token_cache: DashMap<String, TenantContext>,
}

#[tokio::main]
async fn main() {
    let database = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await.expect("Failed connect to database");

    let token_cache = initialize(&pool).await;

    let state = Arc::new(AppState {
        db: pool,
        token_cache
    });

    let router1 = Router::new()
        .route("/tables", get(read_all_tables))
        .route("/tables/:table_name", get(read_table_detail).post(create_table).put(update_table).delete(delete_table))
        .layer(from_fn_with_state(
            state.clone(), table_middleware));

    let router2 = Router::new()
            .route("/api/auth/register", post(register))
            .route("/api/auth/login", post(login))
            .layer(from_fn_with_state(state.clone(), rate_limit));
        

    let app = Router::new()
        .merge(router2)
        .nest("/api", router1)
        .layer(from_fn_with_state(state.clone(), cors_middleware))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();
    println!("🚀 Server Running at port 8001");
    serve(listener, app).await.unwrap();
}