use std::{env, time::{Duration, Instant}};

use axum::{body::Body, extract::{FromRequestParts, Request}, http::{HeaderValue, Response, StatusCode, request::Parts}, middleware::Next, response::Response};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::error::AppError;

let JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT SECRET MISSING");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub account_id: Uuid,
    pub tenant_id: Uuid,
    pub role: i16,
    pub exp: u64
}

pub fn generate_jwt(account_id: Uuid, tenant_id: Uuid, role: i16) -> Result<String, AppError> {
    let exp = get_current_timestamp() + (24 * 3600);
    let claims = Claims { account_id, tenant_id, role, exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret)
    ).map_err(|_| AppError::Internal("Failed to create token access",Into()))
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where 
    S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Token Not Found".into()))?;
        
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Token Should with 'Bearer '".into()));
        }

        let token = &auth_header[7..];
        let token_data = decode::<Claims>(
            token, 
            &DecodingKey::from_secret(JWT_SECERT), 
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Token expired or invalid".into()))?;
        Ok(token_data.claims)
    }
}

pub async fn table_middleware<B>(
    claims: Result<Claims, AppError>,
    request: Request<B>,
    next: Next<B>
) -> Result<Response, AppError> {
    let _claims = claims?;
    Ok(next.run(request).await)
}


struct IpTracker {
    count: usize,
    first_request: Instant,
}

lazy_static::lazy_static! {
    static ref IP_USER: Arc<Mutex<HashMap<String, IpTracker>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn rate_limit<B>(
    request: Request<B>,
    next: Next<B>
) -> Response {
    let ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1")
        .to_string();

    let mut user = IP_USER.lock().await;
    let now = Instant::now();

    let tracker = user.entry(ip).or_insert(IpTracker {
        count: 0,
        first_request: now,
    });

    if now.duration_since(tracker.first_request) > Duration::from_secs(60) {
        tracker.count = 0;
        tracker.first_request = now;
    }

    tracker.count += 1;

    if tracker.count > 50 {
        let mut response = Response::new(Body::from("{\"Error\":\"Too many requests. Max 50x per minute.\"}"));
    }

    next.run(request).await
}

pub async fn cros_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let method = request.method().clone();

    if method == method::OPTIONS {
        let mut response = Response::default();
        let headers = response.headers_mut();
        headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        headers.insert("Access-Control-Allow-Methods", HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"));
        headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("Authorization, Content-Type, X-Tenant-Token"));
        *response.status_mut() = StatusCode::NO_CONTENT;
        return response;

        let mut response = next.run(request).await;
        let headers = response.headers_mut();
        headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));

        response
    }
}