use argon2::{
    PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use thiserror::Error;
use zenki_util::{i32_to_usize, usize_to_i32};

use crate::State;

#[derive(Error, Debug)]
#[error(transparent)]
pub enum VerifyPasswordError {
    Sqlx(#[from] sqlx::Error),
    Argon2(#[from] argon2::password_hash::Error),
}

impl State {
    /// # Errors
    /// when querying the database or creating password hash failed
    pub async fn verify_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<usize>, VerifyPasswordError> {
        let user = sqlx::query!(r"SELECT passwd, uid FROM users WHERE uname = $1", username)
            .fetch_one(&self.db)
            .await?;

        let parsed_hash = argon2::PasswordHash::new(&user.passwd)?;
        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
            .then_some(i32_to_usize(user.uid)))
    }

    /// # Errors
    /// when querying the database or creating password hash failed
    pub async fn verify_password_by_id(
        &self,
        uid: usize,
        password: &str,
    ) -> Result<bool, VerifyPasswordError> {
        let user = sqlx::query!(
            r"SELECT passwd FROM users WHERE uid = $1",
            usize_to_i32(uid)
        )
        .fetch_one(&self.db)
        .await?;

        let parsed_hash = argon2::PasswordHash::new(&user.passwd)?;
        Ok(self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn register(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), VerifyPasswordError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        sqlx::query!(
            r"INSERT INTO users (uname, passwd) VALUES ($1, $2)",
            username,
            password_hash
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn change_password(
        &self,
        id: usize,
        old_passwd: &str,
        passwd: &str,
    ) -> Result<bool, VerifyPasswordError> {
        let user = sqlx::query!(r"SELECT passwd FROM users WHERE uid = $1", usize_to_i32(id))
            .fetch_one(&self.db)
            .await?;

        let parsed_hash = argon2::PasswordHash::new(&user.passwd)?;
        if self
            .argon2
            .verify_password(old_passwd.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Ok(false);
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(passwd.as_bytes(), &salt)?
            .to_string();
        sqlx::query!(
            r"UPDATE users SET passwd = $1 WHERE uid = $2",
            password_hash,
            usize_to_i32(id),
        )
        .execute(&self.db)
        .await?;
        Ok(true)
    }
}
