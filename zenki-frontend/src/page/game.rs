#![allow(clippy::too_many_lines)]
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;

use crate::{
    activity::{is_playing, start_playing, stop_playing},
    auth::get_login_session,
    game::{
        GameError, GameParams, WishlistStatus, add_game_to_wishlist, get_developers_by_game,
        get_game, get_other_games_from_same_developers, get_wishlist_status,
        remove_game_from_wishlist,
    },
    item::get_items,
    review::{get_reviews, post_review},
    route::{DEVELOPER, GAME, ITEM, TAG, USER},
    tag::get_tags,
};

#[component]
pub fn Game() -> impl IntoView {
    let query = use_params::<GameParams>();
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.id.unwrap_or_default())
                .map_err(|_| GameError::InvalidId)
        })
    };
    let game_resource = Resource::new_blocking(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_game(id)
                .await
                .map(|data| data.ok_or(GameError::GameNotFound))
                .map_err(|_| GameError::ServerError),
        }
    });
    let game_view = Suspend::new(async move {
        match game_resource.await {
            Ok(Ok(game)) => {
                Ok(view! {
                    <h2>{game.gname.clone()}</h2>
                    <p><b>Bio: </b>{game.descr.clone().unwrap_or_else(|| String::from("<no description provided>"))}</p>
                    <p><b>Rating: </b>{game.rating.clone()}</p>
                    <p><b>Release Date: </b>{game.release_at.clone().unwrap_or_else(|| String::from("<no release date provided>"))}</p>
                    <p><b>Date Added: </b>{game.created_at.clone().unwrap_or_else(|| String::from("<no added date provided>"))}</p>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    <Title text=game.gname/>
                    <Meta name="description" content=game.descr.unwrap_or_default()/>
                })
            }
            _ => Err(GameError::ServerError),
        }
    });

    let tags_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_tags(id).await.map_err(|_| GameError::ServerError),
        }
    });
    let tags_view = Suspend::new(async move {
        (tags_resource.await).map_or(Err(GameError::ServerError), |tags| {
            Ok(view! {
                <h3>"Tags"</h3>
                <ul>{
                    if tags.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        tags
                            .into_iter()
                            .map(|tag| view! {
                                <li>
                                    <a href=format!("{}/{}", TAG, tag.tname)>{tag.tname.clone()}</a>
                                </li>
                            })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let reviews_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_reviews(id).await.map_err(|_| GameError::ServerError),
        }
    });
    let reviews_view = Suspend::new(async move {
        (reviews_resource.await).map_or(Err(GameError::ServerError), |reviews| {
            Ok(view! {
                <h3>"Reviews"</h3>
                <ul>{
                    if reviews.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        reviews
                            .into_iter()
                            .map(|review| view! {
                                <li>
                                    {review.reviewed_at.unwrap_or_else(|| String::from("<no reviewed timestamp provided>"))} " - "
                                    <b><a href=format!("{}/{}", USER, review.uid)>{" "}{review.uname}{" "}</a></b> " - "
                                    <b>{" ["}{review.rated.unwrap_or_default()} " / 5]"</b> "   "
                                    <em>{" \""}{review.reviewed_text}{"\" "}</em>
                                </li>
                            })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let cookie = get_login_session().0;
    let uid = cookie.get().unwrap_or_default();
    let wishlist_status_resource = Resource::new(
        move || (uid, id()),
        |(uid, gid)| async move {
            match gid {
                Err(e) => Err(e),
                Ok(gid) => get_wishlist_status(uid, gid)
                    .await
                    .map_err(|_| GameError::ServerError),
            }
        },
    );
    let rated = RwSignal::new(String::new());
    let reviewed_text = RwSignal::new(String::new());
    let on_submit_review = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if let Ok(rated_f64) = rated.get().parse() {
                if post_review(
                    uid,
                    id().unwrap_or_default(),
                    rated_f64,
                    reviewed_text.get(),
                )
                .await
                .is_ok()
                {
                    rated.set(String::new());
                    reviewed_text.set(String::new());
                }
            }
        });
    };
    let write_review_view = Suspend::new(async move {
        (wishlist_status_resource.await).map_or(Err(GameError::ServerError), |wishlist_status| {
            Ok(view! {
                <h3>"Write A Review"</h3> {
                if matches!(wishlist_status, WishlistStatus::Owned) {
                    view! {
                        <form on:submit=on_submit_review>
                            <div>
                                <label for="rating">"Rating:"</label>
                                <input
                                    id="rating"
                                    type="number"
                                    min="0"
                                    max="5"
                                    step="0.5"
                                    bind:value=rated
                                />
                            </div>
                            <div>
                                <label for="text">"Text:"</label>
                                <input
                                    id="text"
                                    type="text"
                                    bind:value=reviewed_text
                                />
                            </div>
                            <div>
                                <button type="submit">"Post"</button>
                            </div>
                        </form>
                    }.into_any()
                } else {
                    view! {<p>"You must own this game before writing a review."</p>}.into_any()
                }}
            })
        })
    });

    let on_submit_add_to_wishlist = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let _ = add_game_to_wishlist(uid, id().unwrap_or_default()).await;
        });
    };
    let on_submit_remove_from_wishlist = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let _ = remove_game_from_wishlist(uid, id().unwrap_or_default()).await;
        });
    };
    let add_to_wishlist_view = Suspend::new(async move {
        (wishlist_status_resource.await).map_or(Err(GameError::ServerError), |wishlist_status| {
            Ok(view! {
                <h3>"Wishlist"</h3> {
                match wishlist_status {
                    WishlistStatus::InWishlist => {
                        view! {
                            <form on:submit=on_submit_remove_from_wishlist>
                                 <div>
                                     <button type="submit">"Remove from wishlist"</button>
                                 </div>
                             </form>
                        }.into_any()
                    }
                    WishlistStatus::NotInWishlist => {
                        view! {
                            <form on:submit=on_submit_add_to_wishlist>
                                <div>
                                    <button type="submit">"Add to wishlist"</button>
                                </div>
                            </form>
                        }.into_any()
                    }
                    WishlistStatus::Owned => {
                        view! {<p>"You already owned this game."</p>}.into_any()
                    }
                }}
            })
        })
    });

    let item_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_items(id).await.map_err(|_| GameError::ServerError),
        }
    });
    let items_view = Suspend::new(async move {
        (item_resource.await).map_or(Err(GameError::ServerError), |items| {
            Ok(view! {
                <h3>"Items"</h3>
                <ul>{
                    if items.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        items
                            .into_iter()
                            .map(|item| view! {
                                <li>
                                   <b><a href=format!("{}/{}", ITEM, item.pid)>{" "}{item.descr}{" "}</a></b> " - "
                                   <b>{" ["}{item.price} " USD]"</b>
                                </li>
                            })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let developers_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_developers_by_game(id)
                .await
                .map_err(|_| GameError::ServerError),
        }
    });
    let developers_view = Suspend::new(async move {
        (developers_resource.await).map_or(Err(GameError::ServerError), |devs| {
            Ok(view! {
                <h3>"Developers"</h3>
                <ul>{
                    if devs.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        devs
                            .into_iter()
                            .map(|dev| view! {
                                <li>
                                    <a href=format!("{}/{}", DEVELOPER, dev.did)>{dev.dname}</a>
                                </li>
                            })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let other_games_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_other_games_from_same_developers(id)
                .await
                .map_err(|_| GameError::ServerError),
        }
    });
    let other_games_view = Suspend::new(async move {
        (other_games_resource.await).map_or(Err(GameError::ServerError), |games| {
            Ok(view! {
                <h3>"Other Games from the Same Developers"</h3>
                <ul>{
                    if games.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        games
                            .into_iter()
                            .map(|game| view! {
                                <li>
                                    <a href=format!("{}/{}", GAME, game.gid)>{game.gname}</a>
                                </li>
                            })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    let on_submit_start_playing = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let _ = start_playing(uid, id().unwrap_or_default()).await;
        });
    };
    let on_submit_stop_playing = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let _ = stop_playing(uid, id().unwrap_or_default()).await;
        });
    };
    let is_playing_resource = Resource::new(
        move || (uid, id()),
        |(uid, gid)| async move {
            match gid {
                Err(e) => Err(e),
                Ok(gid) => is_playing(uid, gid)
                    .await
                    .map_err(|_| GameError::ServerError),
            }
        },
    );
    let play_button_view = Suspend::new(async move {
        (is_playing_resource.await).map_or(Err(GameError::ServerError), |is_playing| {
            Ok(view! {
                <h3>"Game Launcher"</h3>{
                if is_playing {
                    view!{ <form on:submit=on_submit_stop_playing>
                         <div>
                             <button type="submit">"Quit"</button>
                         </div>
                    </form> }.into_any()
                } else {
                    view!{ <form on:submit=on_submit_start_playing>
                         <div>
                             <button type="submit">"Play"</button>
                         </div>
                    </form> }.into_any()
                }
            }})
        })
    });

    view! {
        <h1>"Game Info"</h1>
        <Suspense fallback=move || view! { <p>"Loading game..."</p> }>
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
            }>{game_view}</ErrorBoundary>
        </Suspense>
        <Suspense fallback=move || view! { <p>"Loading play button..."</p> }>{play_button_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading developers..."</p> }>{developers_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading tags..."</p> }>{tags_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading items..."</p> }>{items_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading other games..."</p> }>{other_games_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading wishlist..."</p> }>{add_to_wishlist_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading review writer..."</p> }>{write_review_view}</Suspense>
        <Suspense fallback=move || view! { <p>"Loading reviews..."</p> }>{reviews_view}</Suspense>
    }
}
