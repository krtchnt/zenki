#![allow(clippy::too_many_lines)]
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params;

use crate::{
    developer::{DeveloperError, DeveloperParams, get_developer, get_games_from_developer},
    route::GAME,
};

#[component]
pub fn Developer() -> impl IntoView {
    let query = use_params::<DeveloperParams>();
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.id.unwrap_or_default())
                .map_err(|_| DeveloperError::InvalidId)
        })
    };
    let developer_resource = Resource::new_blocking(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(tname) => get_developer(tname)
                .await
                .map(|data| data.ok_or(DeveloperError::DeveloperNotFound))
                .map_err(|_| DeveloperError::ServerError),
        }
    });
    let developer_view = Suspend::new(async move {
        match developer_resource.await {
            Ok(Ok(developer)) => {
                Ok(view! {
                    <h2>{developer.dname.clone()}</h2>
                    <p><b>Description: </b>{developer.descr.clone().unwrap_or_else(|| String::from("<no description provided>"))}</p>
                    <p><b>Created At: </b>{developer.created_at.clone().unwrap_or_else(|| String::from("<no creation date provided>"))}</p>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    <Title text=developer.dname/>
                    <Meta name="description" content=developer.descr.unwrap_or_default()/>
                })
            }
            _ => Err(DeveloperError::ServerError),
        }
    });

    let games_resource = Resource::new(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_games_from_developer(id)
                .await
                .map_err(|_| DeveloperError::ServerError),
        }
    });
    let games_view = Suspend::new(async move {
        (games_resource.await).map_or(Err(DeveloperError::ServerError), |games| {
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
        <h1>"Developer Info"</h1>
        <Suspense fallback=move || view! { <p>"Loading developer..."</p> }>
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
            }>{developer_view}</ErrorBoundary>
        </Suspense>
        <Suspense fallback=move || view! { <p>"Loading games..."</p> }>{games_view}</Suspense>
    }
}
