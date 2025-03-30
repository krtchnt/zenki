use leptos::prelude::*;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct ItemParams {
    pub id: Option<usize>,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemError {
    #[error("Invalid item ID.")]
    InvalidId,
    #[error("Item not found.")]
    ItemNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub pid: i32,
    pub gid: i32,
    pub kind: String,
    pub price: f64,
    pub descr: Option<String>,
    pub created_at: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::Purchase> for Item {
    fn from(value: zenki_backend::Purchase) -> Self {
        Self {
            pid: value.pid,
            gid: value.gid,
            kind: value.purchase_type.to_string(),
            price: value.price,
            descr: value.descr,
            created_at: value.created_at.map(|x| x.to_string()),
        }
    }
}

#[server]
pub async fn get_items(gid: usize) -> Result<Vec<Item>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_purchases(gid)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

#[server]
pub async fn get_item(pid: usize) -> Result<Option<Item>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.query_purchase(pid).await?.map(Into::into))
}
