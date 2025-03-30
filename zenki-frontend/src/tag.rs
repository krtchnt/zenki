use leptos::prelude::*;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::game::Game;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct TagParams {
    pub tname: Option<String>,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagError {
    #[error("Invalid tag ID.")]
    InvalidId,
    #[error("Tag not found.")]
    TagNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Tag {
    pub tname: String,
    pub descr: Option<String>,
    pub category: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::Tag> for Tag {
    fn from(value: zenki_backend::Tag) -> Self {
        Self {
            tname: value.tname,
            descr: value.descr,
            category: value.category,
        }
    }
}

#[server]
pub async fn get_tag(tname: String) -> Result<Option<Tag>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.query_tag(tname).await.map(|x| x.map(Into::into))?)
}

#[server]
pub async fn get_tags(gid: usize) -> Result<Vec<Tag>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_tags(gid)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())?)
}

#[server]
pub async fn get_games_with_tag(tname: String) -> Result<Vec<Game>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_games_with_tag(tname)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())?)
}
