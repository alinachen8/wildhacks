use axum::{
    routing::get,
    routing::post,
    routing::delete,
    routing::patch,
    Router,
    extract::State,
};

use sqlx::postgres::PgPool;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use http::Method;

pub mod routes;
pub mod models;
pub mod database;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    env::set_var("DATABASE_URL", "postgresql://wecu:MwrK5eYM29e1Es99Uts5tw@wildhacks-14336.5xj.gcp-us-central1.cockroachlabs.cloud:26257/data?sslmode=verify-full");
    env::set_var("DB_URL", "postgresql://wecu:MwrK5eYM29e1Es99Uts5tw@wildhacks-14336.5xj.gcp-us-central1.cockroachlabs.cloud:26257/data?sslmode=verify-full");

    let state = AppState {
        pool: PgPool::connect(env::var("DB_URL").unwrap().as_str()).await.unwrap()
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any);

    let app = Router::new()
        .route("/listings", post(routes::listings::post))
        .route("/listings", get(routes::listings::get))
        .route("/listings", delete(routes::listings::delete))
        .route("/listings", patch(routes::listings::patch))
        .route("/users", post(routes::users::post))
        .route("/users", get(routes::users::get))
        .route("/users", patch(routes::users::patch))
        .route("/portfolio/sell", patch(routes::portfolio::sell))
        .route("/portfolio/buy", patch(routes::portfolio::buy))
        .route("/portfolio", get(routes::portfolio::get_portfolio))
        .with_state(state)
        .layer(cors)
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}