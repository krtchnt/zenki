use leptos::{prelude::*, reactive::spawn, task::spawn_local};
use leptos_meta::Title;

use crate::auth::login;

#[component]
pub fn Login() -> impl IntoView {
    let (cookie, set_cookie) = crate::auth::get_login_session();
    if cookie.get().is_some() {
        spawn(async {
            let _ = crate::route::redirect_to_main().await;
        });
    }

    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if login(username.get().clone(), password.get().clone(), None)
                .await
                .is_ok()
            {
                set_cookie.set(Some(username.get()));
            } else {
                username.set(String::new());
                password.set(String::new());
            }
        });
    };

    let on_signup_click = |_| {
        spawn_local(async move {
            let _ = crate::route::redirect_to_register().await;
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
        <div>
            <button on:click=on_signup_click>"Sign Up"</button>
        </div>
    }
}
