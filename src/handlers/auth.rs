use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::env;

use crate::models::user::{RegisterRequest, LoginRequest, AuthResponse, User};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // user_id as string
    exp: i64,
}

pub async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let password_hash = match hash(payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let user_id = Uuid::new_v4();
    let user_id_bytes = user_id.as_bytes().to_vec();

    let result = sqlx::query(
        "INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)"
    )
    .bind(user_id_bytes)
    .bind(payload.username)
    .bind(password_hash)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.is_unique_violation() {
                    return (StatusCode::BAD_REQUEST, "Username already exists").into_response();
                }
            }
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash FROM users WHERE username = ?"
    )
    .bind(&payload.username)
    .fetch_optional(&pool)
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if !verify(payload.password, &user.password_hash).unwrap_or(false) {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // Generate JWT
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let user_id = user.id.to_string();
    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(t) => t,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(AuthResponse { token }).into_response()
}
