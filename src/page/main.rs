use leptos::{prelude::*, task};
use leptos_meta::Title;

#[component]
pub fn Main() -> impl IntoView {
    let (cookie, set_cookie) = crate::auth::get_login_session();
    if cookie.get().is_none() {
        task::spawn(async {
            let _ = crate::route::redirect_to_login().await;
        });
    }

    let on_signout_click = move |_| {
        set_cookie.set(None);
        task::spawn_local(async {
            let _ = crate::route::redirect_to_login().await;
        });
    };

    view! {
        <Title text="Zenki"/>
        <h1>"Zenki"</h1>
        <div>
            <p>Logged in as: <b>{move || cookie.get().unwrap_or_default()}</b></p>
        </div>
        <div>
            <button on:click=on_signout_click>"Sign Out"</button>
        </div>
    }
}
