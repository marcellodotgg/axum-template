use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use crate::auth::GoogleUser;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i64,
    pub picture: Option<String>,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
    pub verified: bool,
}

impl User {
    pub fn from_google(user: GoogleUser) -> Self {
        User{
            id: 0,
            email: user.email,
            given_name: user.given_name,
            family_name: user.family_name,
            picture: Some(user.picture),
            verified: user.verified_email,
        }
    }

    pub async fn find_by_email(pool: &SqlitePool, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email).fetch_one(pool).await
    }

    pub async fn create(&self, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query!(
            "insert into users (email, given_name, family_name, picture, verified) values (?, ?, ?, ?, ?)",
            self.email,
            self.given_name,
            self.family_name,
            self.picture,
            self.verified
        ).execute(&mut *tx).await?;

        let user = sqlx::query_as!(User, "select * from users where email = ?", self.email).fetch_one(&mut *tx).await?;
        tx.commit().await?;

        Ok(user)
    }
}