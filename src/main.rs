mod models;
mod handlers;
mod middleware;

use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware::from_fn,
};
use std::net::SocketAddr;
use sqlx::mysql::MySqlPoolOptions;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Database connection
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Auth routes
    let auth_routes = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login));

    // Protected article management routes
    let article_mgmt_routes = Router::new()
        .route("/", get(handlers::article::list_user_articles))
        .route("/", post(handlers::article::create_article))
        .route("/:id", put(handlers::article::update_article))
        .route("/:id", delete(handlers::article::delete_article))
        .layer(from_fn(middleware::auth::auth_middleware));

    // Build application with routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, Article Recorder!" }))
        .nest("/api/auth", auth_routes)
        .nest("/api/articles", article_mgmt_routes)
        .route("/:slug", get(handlers::article::get_article_by_slug))
        .layer(from_fn(middleware::auth::optional_auth_middleware))
        .with_state(pool);

    // Run the server
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
