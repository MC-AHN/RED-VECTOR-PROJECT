use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
    pub tenant_name_slug: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub account_id: Uuid,
    pub tenant_id: Uuid,
    pub tenant_name_slug: String,
    pub client_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}