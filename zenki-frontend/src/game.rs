use leptos::prelude::*;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::developer::Developer;

#[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
pub struct GameParams {
    pub id: Option<usize>,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameError {
    #[error("Invalid game ID.")]
    InvalidId,
    #[error("Game not found.")]
    GameNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub gid: usize,
    pub gname: String,
    pub descr: Option<String>,
    pub rating: String,
    pub release_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameRef {
    pub gid: usize,
    pub gname: String,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::GameRef> for GameRef {
    fn from(value: zenki_backend::GameRef) -> Self {
        Self {
            gid: zenki_util::i32_to_usize(value.gid),
            gname: value.gname,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum WishlistStatus {
    NotInWishlist,
    InWishlist,
    Owned,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::Game> for Game {
    fn from(value: zenki_backend::Game) -> Self {
        Self {
            gid: zenki_util::i32_to_usize(value.gid),
            gname: value.gname,
            descr: value.descr,
            rating: value.rating.to_string(),
            release_at: value.release_at.map(|x| x.to_string()),
            created_at: value.created_at.map(|x| x.to_string()),
        }
    }
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::WishlistStatus> for WishlistStatus {
    fn from(value: zenki_backend::WishlistStatus) -> Self {
        match value {
            zenki_backend::WishlistStatus::NotInWishlist => Self::NotInWishlist,
            zenki_backend::WishlistStatus::InWishlist => Self::InWishlist,
            zenki_backend::WishlistStatus::Owned => Self::Owned,
        }
    }
}

#[server]
pub async fn list_games() -> Result<Vec<Game>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_all_games()
        .await
        .map(|x| x.into_iter().map(Into::into).collect())?)
}

#[server]
pub async fn get_game(id: usize) -> Result<Option<Game>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.query_game(id).await.map(|x| x.map(Into::into))?)
}

#[server]
pub async fn get_wishlist(id: usize) -> Result<Vec<Game>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_wishlist(id)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())?)
}

#[server]
pub async fn get_library(id: usize) -> Result<Vec<Game>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_library(id)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())?)
}

#[server]
pub async fn get_wishlist_status(uid: usize, gid: usize) -> Result<WishlistStatus, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.get_wishlist_status(uid, gid).await.map(Into::into)?)
}

#[server]
pub async fn add_game_to_wishlist(uid: usize, gid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.add_game_to_wishlist(uid, gid).await?)
}

#[server]
pub async fn remove_game_from_wishlist(uid: usize, gid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.remove_game_from_wishlist(uid, gid).await?)
}

#[server]
pub async fn get_other_games_from_same_developers(
    gid: usize,
) -> Result<Vec<GameRef>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_other_games_from_same_developers(gid)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

#[server]
pub async fn get_developers_by_game(gid: usize) -> Result<Vec<Developer>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_developers_by_game(gid)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}
