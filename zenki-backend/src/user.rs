use serde::{Deserialize, Serialize};
use time::{Date, PrimitiveDateTime, error::Parse, macros::format_description};
use zenki_util::usize_to_i32;

use crate::State;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct User {
    pub uid: i32,
    pub uname: String,
    pub passwd: String,
    pub email: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
    pub bio: Option<String>,
    pub birth_date: Option<Date>,
}

impl State {
    /// # Errors
    /// when querying the database failed
    pub async fn query_user(&self, id: usize) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            r"SELECT * FROM users WHERE uid = $1",
            usize_to_i32(id)
        )
        .fetch_optional(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_users(&self) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(User, r"SELECT * FROM users",)
            .fetch_all(&self.db)
            .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn delete_user(&self, id: usize) -> sqlx::Result<()> {
        sqlx::query!(r"DELETE FROM users WHERE uid = $1", usize_to_i32(id))
            .execute(&self.db)
            .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn update_username(&self, id: usize, name: &str) -> sqlx::Result<()> {
        sqlx::query!(
            r"UPDATE users SET uname = $1 WHERE uid = $2",
            name,
            usize_to_i32(id),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn update_bio(&self, id: usize, bio: Option<&str>) -> sqlx::Result<()> {
        sqlx::query!(
            r"UPDATE users SET bio = $1 WHERE uid = $2",
            bio,
            usize_to_i32(id),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn update_email(&self, id: usize, email: Option<&str>) -> sqlx::Result<()> {
        sqlx::query!(
            r"UPDATE users SET email = $1 WHERE uid = $2",
            email,
            usize_to_i32(id),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn update_birth_date(&self, id: usize, birth_date: Option<Date>) -> sqlx::Result<()> {
        sqlx::query!(
            r"UPDATE users SET birth_date = $1 WHERE uid = $2",
            birth_date,
            usize_to_i32(id),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
}

/// # Errors
/// when parsing the date failed
pub fn parse_html_date(date: &str) -> Result<Date, Parse> {
    Date::parse(date, format_description!("[year]-[month]-[day]"))
}
