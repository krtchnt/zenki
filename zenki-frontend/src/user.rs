use leptos::prelude::*;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
pub struct UserParams {
    pub id: Option<usize>,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserError {
    #[error("Invalid user ID.")]
    InvalidId,
    #[error("User not found.")]
    UserNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub uid: usize,
    pub uname: String,
    pub passwd: String,
    pub email: Option<String>,
    pub created_at: Option<String>,
    pub bio: Option<String>,
    pub birth_date: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::User> for User {
    fn from(value: zenki_backend::User) -> Self {
        Self {
            uid: zenki_util::i32_to_usize(value.uid),
            uname: value.uname,
            bio: value.bio,
            passwd: value.passwd,
            email: value.email,
            created_at: value.created_at.map(|x| x.to_string()),
            birth_date: value.birth_date.map(|x| x.to_string()),
        }
    }
}

#[server]
pub async fn get_user(id: usize) -> Result<Option<User>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.query_user(id).await?.map(Into::into))
}

#[server]
pub async fn update_bio(id: usize, bio: String) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .update_bio(id, (!bio.is_empty()).then_some(bio).as_deref())
        .await?)
}

#[server]
pub async fn update_username(id: usize, name: String) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.update_username(id, &name).await?)
}

#[server]
pub async fn update_email(id: usize, email: String) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .update_email(id, (!email.is_empty()).then_some(email).as_deref())
        .await?)
}

#[server]
pub async fn update_birth_date(id: usize, birth_date: String) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .update_birth_date(
            id,
            if birth_date.is_empty() {
                None
            } else {
                Some(zenki_backend::parse_html_date(&birth_date)?)
            },
        )
        .await?)
}

#[server]
pub async fn delete_user(id: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.delete_user(id).await?)
}

#[server]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_users()
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}
