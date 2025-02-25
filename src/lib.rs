use sqlx::SqlitePool;

pub mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}
