use leptos::prelude::*;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[cfg(feature = "ssr")]
use zenki_util::i32_to_usize;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct TransactionParams {
    pub id: Option<usize>,
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionError {
    #[error("Invalid transaction ID.")]
    InvalidId,
    #[error("Transaction not found.")]
    TransactionNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TransactionHistory {
    pub tid: usize,
    pub gid: usize,
    pub gname: String,
    pub pid: usize,
    pub p_descr: Option<String>,
    pub bought_at: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::TransactionHistory> for TransactionHistory {
    fn from(value: zenki_backend::TransactionHistory) -> Self {
        Self {
            tid: i32_to_usize(value.tid),
            gid: i32_to_usize(value.gid),
            gname: value.gname,
            pid: i32_to_usize(value.pid),
            p_descr: value.p_descr,
            bought_at: value.bought_at.map(|x| x.to_string()),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RichTransaction {
    pub tid: usize,
    pub uid: usize,
    pub receiver_uid: Option<usize>,
    pub pid: usize,
    pub payment_method: String,
    pub amount: usize,
    pub bought_at: Option<String>,
    pub status: Option<String>,
    pub s_uname: String,
    pub r_uname: String,
    pub p_descr: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::RichTransaction> for RichTransaction {
    fn from(value: zenki_backend::RichTransaction) -> Self {
        Self {
            tid: i32_to_usize(value.tid),
            uid: i32_to_usize(value.uid),
            receiver_uid: value.receiver_uid.map(i32_to_usize),
            pid: i32_to_usize(value.pid),
            payment_method: value.payment_method.to_string(),
            amount: i32_to_usize(value.amount),
            bought_at: value.bought_at.map(|x| x.to_string()),
            status: value.status,
            s_uname: value.s_uname,
            r_uname: value.r_uname,
            p_descr: value.p_descr,
        }
    }
}

#[server]
pub async fn create_transaction(
    uid: usize,
    pid: usize,
    ruid: usize,
    payment_method: String,
    amount: usize,
) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .create_transaction(uid, pid, ruid, payment_method.parse()?, amount)
        .await?)
}

#[server]
pub async fn get_transaction(tid: usize) -> Result<Option<RichTransaction>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.query_rich_transaction(tid).await?.map(Into::into))
}

#[server]
pub async fn get_transaction_history(uid: usize) -> Result<Vec<TransactionHistory>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_transaction_history(uid)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}
