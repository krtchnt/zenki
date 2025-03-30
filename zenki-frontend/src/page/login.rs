use leptos::{prelude::*, task::spawn_local};
use leptos_meta::Title;

use crate::{auth::login, route::REGISTER};

#[component]
pub fn Login() -> impl IntoView {
    let set_cookie = crate::auth::get_login_session().1;
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if let Ok(id) = login(username.get().clone(), password.get().clone()).await {
                set_cookie.set(Some(id));
                let _ = crate::route::redirect_to_main().await;
            } else {
                username.set(String::new());
                password.set(String::new());
            }
        });
    };

    view! {
        <Title text="Sign In"/>
        <h1>"Sign In"</h1>
        <form on:submit=on_submit>
            <div>
                <label for="username">"Username:"</label>
                <input
                    id="username"
                    type="text"
                    bind:value=username
                />
            </div>
            <div>
                <label for="password">"Password:"</label>
                <input
                    id="password"
                    type="password"
                    bind:value=password
                />
            </div>
            <div>
                <button type="submit">"Login"</button>
            </div>
        </form>
        <form action=REGISTER>
            <input type="submit" value="Sign Up" />
        </form>
    }
}
