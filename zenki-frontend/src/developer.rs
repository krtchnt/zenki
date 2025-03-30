use leptos::prelude::*;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::game::Game;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct DeveloperParams {
    pub id: Option<usize>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Developer {
    pub did: usize,
    pub dname: String,
    pub descr: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeveloperError {
    #[error("Invalid developer ID.")]
    InvalidId,
    #[error("Developer not found.")]
    DeveloperNotFound,
    #[error("Server error.")]
    ServerError,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::Developer> for Developer {
    fn from(value: zenki_backend::Developer) -> Self {
        Self {
            did: zenki_util::i32_to_usize(value.did),
            dname: value.dname,
            descr: value.descr,
            created_at: value.created_at.map(|x| x.to_string()),
        }
    }
}

#[server]
pub async fn get_developer(gid: usize) -> Result<Option<Developer>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.query_developer(gid).await?.map(Into::into))
}

#[server]
pub async fn get_games_from_developer(did: usize) -> Result<Vec<Game>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_games_from_developer(did)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}
