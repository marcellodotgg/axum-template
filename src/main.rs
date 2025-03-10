use axum::{routing::get, Router};
use axum_template::{AppState, routes::api};
use sqlx::{Pool, Sqlite, SqlitePool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // load environment variables

    let state = AppState {
        db: init_db().await,
    };

    let ping_routes = Router::new().route("/ping", get(api::ping::ping));
    let auth_routes = Router::new()
        .route("/auth/google", get(api::auth::google::oauth))
        .route("/auth/google/callback", get(api::auth::google::oauth_callback));

    let app = Router::new().merge(ping_routes).merge(auth_routes).with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn init_db() -> Pool<Sqlite> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to the database")
}
