use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{Meta, Title};

use crate::{
    game::list_games,
    route::{GAME, USER},
    user::{User, UserError, get_user},
};

#[component]
pub fn Main() -> impl IntoView {
    let (get_cookie, set_cookie) = crate::auth::get_login_session();
    // load the games
    let games = Resource::new(|| (), |()| list_games());
    let games = move || {
        games
            .get()
            .map(Result::unwrap_or_default)
            .unwrap_or_default()
    };

    let games_count = Resource::new(|| (), |()| list_games());
    let games_count = Resource::new(
        || (),
        move |()| async move { games_count.await.as_ref().map(Vec::len).unwrap_or(0) },
    );

    let id = move || get_cookie.get().ok_or(UserError::ServerError);
    let user_resource = Resource::new_blocking(id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => get_user(id)
                .await
                .map(|data| data.ok_or(UserError::UserNotFound))
                .map_err(|_| UserError::ServerError),
        }
    });

    let on_signout_click = move |_| {
        set_cookie.set(None);
        spawn_local(async {
            let _ = crate::route::redirect_to_login().await;
        });
    };
    let user_view = Suspend::new(async move {
        match user_resource.await {
            Ok(Ok(User { uid, uname, .. })) => Ok(view! {
                <div>
                    <p>Logged in as: <a href={format!("{USER}/{uid}")}><b>{uname.clone()}</b></a></p>
                </div>
                <div>
                    <button on:click=on_signout_click>"Sign Out"</button>
                </div>

                <Title text=uname />
                <Meta name="description" content=uid.to_string() />
            }),
            _ => Err(UserError::ServerError),
        }
    });

    view! {
        <h1>"Zenki"</h1>
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
        <Suspense fallback=move || view! { <p>"Loading games..."</p> }>
            <p>Total Games: <b>{Suspend::new(async move { games_count.await })}</b></p>
        </Suspense>
        <Suspense fallback=move || view! { <p>"Loading games..."</p> }>
            <table>
                <tr>
                    <th>Title</th>
                    <th>Description</th>
                    <th>Rating</th>
                    <th>Release Date</th>
                    <th>Date Added</th>
                </tr>
                <For each=games key=|game| game.gid let:game>
                    <tr>
                        <td><a href=format!("{GAME}/{}", game.gid)>{game.gname.clone()}</a></td>
                        <td>{game.descr.unwrap_or_else(|| String::from("<no description provided>"))}</td>
                        <td>{game.rating}</td>
                        <td>{game.release_at.unwrap_or_else(|| String::from("<no release date provided>"))}</td>
                        <td>{game.created_at.unwrap_or_else(|| String::from("<no added date provided>"))}</td>
                    </tr>
                </For>
            </table>
        </Suspense>
    }
}
