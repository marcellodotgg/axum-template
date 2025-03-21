pub mod routes;
pub mod auth;
pub mod utils;
mod user;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}
