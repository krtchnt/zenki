#![allow(clippy::too_many_lines)]
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;

use crate::{
    route::GAME,
    tag::{TagError, TagParams, get_games_with_tag, get_tag},
};

#[component]
pub fn Tag() -> impl IntoView {
    let query = use_params::<TagParams>();
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.tname.clone().unwrap_or_default())
                .map_err(|_| TagError::InvalidId)
        })
    };
    let tag_resource = Resource::new_blocking(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(tname) => get_tag(tname)
                .await
                .map(|data| data.ok_or(TagError::TagNotFound))
                .map_err(|_| TagError::ServerError),
        }
    });
    let tag_view = Suspend::new(async move {
        match tag_resource.await {
            Ok(Ok(tag)) => {
                Ok(view! {
                    <h2>{tag.tname.clone()}</h2>
                    <p><b>Description: </b>{tag.descr.clone().unwrap_or_else(|| String::from("<no description provided>"))}</p>
                    <p><b>Category: </b>{tag.category.clone().unwrap_or_else(|| String::from("<no category provided>"))}</p>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    <Title text=tag.tname/>
                    <Meta name="description" content=tag.descr.unwrap_or_default()/>
                })
            }
            _ => Err(TagError::ServerError),
        }
    });

    let games_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_games_with_tag(id)
                .await
                .map_err(|_| TagError::ServerError),
        }
    });
    let games_view = Suspend::new(async move {
        (games_resource.await).map_or(Err(TagError::ServerError), |games| {
            Ok(view! {
                <h3>"Games"</h3>
                <ul>{
                    if games.is_empty() {
                        view! {<p>"<empty>"</p>}.into_any()
                    } else {
                        games
                            .into_iter()
                            .map(|game| view! {
                                <li>
                                    <a href=format!("{}/{}", GAME, game.gid)>{game.gname.clone()}</a>
                                </li>
                            })
                            .collect_view().into_any()
                    }
                }</ul>
            })
        })
    });

    view! {
        <h1>"Tag Info"</h1>
        <Suspense fallback=move || view! { <p>"Loading tag..."</p> }>
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
            }>{tag_view}</ErrorBoundary>
        </Suspense>
        <Suspense fallback=move || view! { <p>"Loading games..."</p> }>{games_view}</Suspense>
    }
}
