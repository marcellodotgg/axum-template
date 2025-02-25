use sqlx::SqlitePool;

pub mod handler;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}
