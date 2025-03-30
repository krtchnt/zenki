use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Serialize, Deserialize, Clone)]
pub enum FriendshipStatus {
    NotFriends,
    RequestSent,
    RequestReceived,
    Friends,
}

#[server]
pub async fn get_friendship_status(
    uid: usize,
    fid: usize,
) -> Result<FriendshipStatus, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .get_friendship_status(uid, fid)
        .await
        .map(|x| match x {
            zenki_backend::FriendshipStatus::NotFriends => FriendshipStatus::NotFriends,
            zenki_backend::FriendshipStatus::RequestSent => FriendshipStatus::RequestSent,
            zenki_backend::FriendshipStatus::RequestReceived => FriendshipStatus::RequestReceived,
            zenki_backend::FriendshipStatus::Friends => FriendshipStatus::Friends,
        })?)
}

#[server]
pub async fn accept_friend_request(uid: usize, fid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.accept_friend_request(uid, fid).await?)
}

#[server]
pub async fn decline_friend_request(uid: usize, fid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.decline_friend_request(uid, fid).await?)
}

#[server]
pub async fn send_friend_request(uid: usize, fid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.send_friend_request(uid, fid).await?)
}

#[server]
pub async fn cancel_friend_request(uid: usize, fid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.cancel_friend_request(uid, fid).await?)
}

#[server]
pub async fn remove_friend(uid: usize, fid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.remove_friend(uid, fid).await?)
}

#[server]
pub async fn query_friends(uid: usize) -> Result<Vec<User>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_friends(uid)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())?)
}
