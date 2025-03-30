#![allow(clippy::too_many_lines)]
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;

use crate::{
    activity::get_game_activity,
    friendship::{
        AcceptFriendRequest, CancelFriendRequest, DeclineFriendRequest, FriendshipStatus,
        RemoveFriend, SendFriendRequest, get_friendship_status, query_friends,
    },
    game::{get_library, get_wishlist},
    route::{ACCOUNT, GAME, ITEM, TRANSACTION, USER},
    transaction::get_transaction_history,
    user::{UserError, UserParams, get_user},
};

#[component]
pub fn User() -> impl IntoView {
    let query = use_params::<UserParams>();
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.id.unwrap_or_default())
                .map_err(|_| UserError::InvalidId)
        })
    };
    let user_resource = Resource::new_blocking(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_user(id)
                .await
                .map(|data| data.ok_or(UserError::UserNotFound))
                .map_err(|_| UserError::ServerError),
        }
    });
    let user_view = Suspend::new(async move {
        match user_resource.await {
            Ok(Ok(user)) => {
                Ok(view! {
                    <h2>{user.uname.clone()}</h2>
                    <p><b>Bio: </b>{user.bio.clone().unwrap_or_else(|| String::from("<no bio provided>"))}</p>
                    <p><b>Created At: </b>{user.created_at.unwrap_or_else(|| String::from("<no creation time provided>"))}</p>
                    <p><b>Email: </b>{user.email.unwrap_or_else(|| String::from("<no email provided>"))}</p>
                    <p><b>Birth Date: </b>{user.birth_date.unwrap_or_else(|| String::from("<no birth date provided>"))}</p>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    <Title text=user.uname/>
                    <Meta name="description" content=user.bio.unwrap_or_default()/>
                })
            }
            _ => Err(UserError::ServerError),
        }
    });

    let curr_id = move || crate::auth::get_login_session().0.get();
    let friendship_resource = Resource::new_blocking(
        move || (curr_id(), id()),
        |(uid, fid)| async move {
            match (uid, fid) {
                (_, Err(e)) => Err(e),
                (None, _) => Err(UserError::ServerError),
                (Some(uid), Ok(fid)) => Ok((
                    uid,
                    fid,
                    if uid == fid {
                        None
                    } else {
                        Some(
                            get_friendship_status(uid, fid)
                                .await
                                .map_err(|_| UserError::ServerError)?,
                        )
                    },
                )),
            }
        },
    );

    let send_friend_request_act = ServerAction::<SendFriendRequest>::new();
    let cancel_friend_request_act = ServerAction::<CancelFriendRequest>::new();
    let accept_friend_request_act = ServerAction::<AcceptFriendRequest>::new();
    let decline_friend_request_act = ServerAction::<DeclineFriendRequest>::new();
    let remove_friend_act = ServerAction::<RemoveFriend>::new();

    let friendship_view = Suspend::new(async move {
        match friendship_resource.await {
            Ok((uid, fid, status)) => Ok(view! {
                <div>
                    {match status {
                        Some(FriendshipStatus::NotFriends) => view! {
                            <ActionForm action=send_friend_request_act>
                                <input type="hidden" name="uid" value=uid/>
                                <input type="hidden" name="fid" value=fid/>
                                <button class="btn btn-blue">
                                    "Send Friend Request"
                                </button>
                            </ActionForm>
                        }.into_any(),
                        Some(FriendshipStatus::RequestSent) => view! {
                            <ActionForm action=cancel_friend_request_act>
                                <input type="hidden" name="uid" value=uid/>
                                <input type="hidden" name="fid" value=fid/>
                                <button class="btn btn-red">
                                    "Cancel Friend Request"
                                </button>
                            </ActionForm>
                        }.into_any(),
                        Some(FriendshipStatus::RequestReceived) => view! {
                            <ActionForm action=accept_friend_request_act>
                                <input type="hidden" name="uid" value=uid/>
                                <input type="hidden" name="fid" value=fid/>
                                <button class="btn btn-green">
                                    "Accept Friend Request"
                                </button>
                            </ActionForm>
                            <ActionForm action=decline_friend_request_act>
                                <input type="hidden" name="uid" value=uid/>
                                <input type="hidden" name="fid" value=fid/>
                                <button class="btn btn-red">
                                    "Decline Friend Request"
                                </button>
                            </ActionForm>
                        }.into_any(),
                        Some(FriendshipStatus::Friends) => view! {
                            <ActionForm action=remove_friend_act>
                                <input type="hidden" name="uid" value=uid/>
                                <input type="hidden" name="fid" value=fid/>
                                <button class="btn btn-red">
                                    "Remove Friend"
                                </button>
                            </ActionForm>
                        }.into_any(),
                        None => {
                            view! {
                                <form action=ACCOUNT>
                                    <input type="submit" value="Edit Account" />
                                </form>
                            }.into_any()
                        }
                    }}
                </div>
            }),
            _ => Err(UserError::ServerError),
        }
    });

    let friends_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => query_friends(id).await.map_err(|_| UserError::ServerError),
        }
    });
    let friends_view = Suspend::new(async move {
        (friends_resource.await).map_or(Err(UserError::ServerError), |friends| {
            Ok(view! {
                <h3>"Friends"</h3>
                <ul>{
                    if friends.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        friends
                            .into_iter()
                            .map(|user| view! { <li><a href=format!("{}/{}", USER, user.uid)>{user.uname}</a></li> })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let wishlist_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_wishlist(id).await.map_err(|_| UserError::ServerError),
        }
    });
    let wishlist_view = Suspend::new(async move {
        (wishlist_resource.await).map_or(Err(UserError::ServerError), |games| {
            Ok(view! {
                <h3>"Wishlist"</h3>
                <ul>{
                    if games.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        games
                            .into_iter()
                            .map(|user| view! { <li><a href=format!("{}/{}", GAME, user.gid)>{user.gname}</a></li> })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let library_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_library(id).await.map_err(|_| UserError::ServerError),
        }
    });
    let library_view = Suspend::new(async move {
        (library_resource.await).map_or(Err(UserError::ServerError), |games| {
            Ok(view! {
                <h3>"Library"</h3>
                <ul>{
                    if games.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        games
                            .into_iter()
                            .map(|user| view! { <li><a href=format!("{}/{}", GAME, user.gid)>{user.gname}</a></li> })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let transactions_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_transaction_history(id)
                .await
                .map_err(|_| UserError::ServerError),
        }
    });
    let transactions_view = Suspend::new(async move {
        (transactions_resource.await).map_or(Err(UserError::ServerError), |txs| {
            Ok(view! {
                <h3>"Transaction History"</h3>
                <ul>{
                    if txs.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        txs
                            .into_iter()
                            .map(|tx| view! {
                                <li>
                                    <a href=format!("{}/{}", TRANSACTION, tx.tid)>
                                        {tx.bought_at.unwrap_or_else(|| String::from("<no bought timestamp provided>"))}
                                    </a>
                                    {" | "}
                                    <a href=format!("{}/{}", GAME, tx.gid)><b>{tx.gname}</b></a>
                                    {" | "}
                                    <a href=format!("{}/{}", ITEM, tx.pid)>{tx.p_descr}</a>
                                </li>
                            })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let activity_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_game_activity(id)
                .await
                .map_err(|_| UserError::ServerError),
        }
    });
    let activity_view = Suspend::new(async move {
        (activity_resource.await).map_or(Err(UserError::ServerError), |acts| {
            Ok(view! {
                <h3>"Recent Game Activities"</h3>
                <ul>{
                    if acts.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        acts
                            .into_iter()
                            .map(|act| view! {
                                <li>
                                    {act.startplay_at}
                                    {" | "}
                                    <a href=format!("{}/{}", GAME, act.gid)><b>{act.gname}</b></a>
                                    {" | "}
                                    {
                                        act.duration.map_or_else(|| view! {
                                            <b><u>"Currently Playing"</u></b>
                                        }.into_any(), |dt| view! {
                                            <b>"Played For: "</b>{dt}
                                        }.into_any())
                                    }
                                </li>
                            })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    view! {
        <h1>"User Profile"</h1>
        <Suspense fallback=move || view! { <p>"Loading user..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, error)| view! { <li>{error.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}
                        </ul>
                    </div>
                }
            }>{user_view}</ErrorBoundary>
        </Suspense>
        <Transition fallback=move || view! { <p>"Loading friendship..."</p> }>{friendship_view}</Transition>
        <Suspense fallback=move || view! { <p>"Loading friends..."</p> }>{friends_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading library..."</p> }>{library_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading wishlist..."</p> }>{wishlist_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading activity..."</p> }>{activity_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading transaction history..."</p> }>{transactions_view}</Suspense>
    }
}
