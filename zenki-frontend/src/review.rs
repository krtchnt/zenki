use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewError {
    #[error("Invalid review ID.")]
    InvalidId,
    #[error("Review not found.")]
    ReviewNotFound,
    #[error("Server error.")]
    ServerError,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Review {
    pub rid: usize,
    pub uid: usize,
    pub uname: String,
    pub rated: Option<f64>,
    pub reviewed_text: Option<String>,
    pub reviewed_at: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::Review> for Review {
    fn from(value: zenki_backend::Review) -> Self {
        use zenki_util::i32_to_usize;
        Self {
            rid: i32_to_usize(value.rid),
            uid: i32_to_usize(value.uid),
            uname: value.uname,
            rated: value.rated,
            reviewed_text: value.reviewed_text,
            reviewed_at: value.reviewed_at.map(|x| x.to_string()),
        }
    }
}

#[server]
pub async fn get_reviews(id: usize) -> Result<Vec<Review>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_reviews(id)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())?)
}

#[server]
pub async fn post_review(
    uid: usize,
    gid: usize,
    rated: f64,
    reviewed_text: String,
) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.post_review(uid, gid, rated, reviewed_text).await?)
}
