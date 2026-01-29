use axum::{
    extract::{State, Path, Extension},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::article::{Article, CreateArticleRequest, UpdateArticleRequest};
use crate::middleware::auth::Claims;

pub async fn list_user_articles(
    State(pool): State<MySqlPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let author_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id.as_bytes().to_vec(),
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE author_id = ? ORDER BY created_at DESC"
    )
    .bind(author_id)
    .fetch_all(&pool)
    .await;

    match articles {
        Ok(as_) => Json(as_).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_article(
    State(pool): State<MySqlPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateArticleRequest>,
) -> impl IntoResponse {
    let author_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id.as_bytes().to_vec(),
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let result = sqlx::query(
        "INSERT INTO articles (author_id, slug, title, content, is_public) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(author_id)
    .bind(payload.slug)
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.is_public)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_article(
    State(pool): State<MySqlPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateArticleRequest>,
) -> impl IntoResponse {
    let author_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id.as_bytes().to_vec(),
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Check ownership
    let article = sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match article {
        Ok(Some(a)) if a.author_id == author_id => {
            let result = sqlx::query(
                "UPDATE articles SET title = COALESCE(?, title), content = COALESCE(?, content), is_public = COALESCE(?, is_public) WHERE id = ?"
            )
            .bind(payload.title)
            .bind(payload.content)
            .bind(payload.is_public)
            .bind(id)
            .execute(&pool)
            .await;

            match result {
                Ok(_) => StatusCode::OK.into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        },
        Ok(Some(_)) => StatusCode::FORBIDDEN.into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_article(
    State(pool): State<MySqlPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let author_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id.as_bytes().to_vec(),
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let result = sqlx::query("DELETE FROM articles WHERE id = ? AND author_id = ?")
        .bind(id)
        .bind(author_id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_article_by_slug(
    State(pool): State<MySqlPool>,
    claims: Option<Extension<Claims>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let article = sqlx::query_as::<_, Article>("SELECT * FROM articles WHERE slug = ?")
        .bind(slug)
        .fetch_optional(&pool)
        .await;

    match article {
        Ok(Some(a)) => {
            // Check if public or author
            if a.is_public == 1 {
                return Json(a).into_response();
            }

            if let Some(Extension(c)) = claims {
                if let Ok(user_id) = Uuid::parse_str(&c.sub) {
                    if a.author_id == user_id.as_bytes().to_vec() {
                        return Json(a).into_response();
                    }
                }
            }
            
            StatusCode::NOT_FOUND.into_response()
        },
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
