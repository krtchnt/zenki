#![allow(clippy::too_many_lines)]
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::Title;

use crate::{
    auth::{change_password, login_by_id},
    route::redirect_to_login,
    user::{delete_user, update_bio, update_birth_date, update_email, update_username},
};

#[component]
pub fn Account() -> impl IntoView {
    let (cookie, set_cookie) = crate::auth::get_login_session();
    let username = RwSignal::new(String::new());
    let bio = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let birth_date = RwSignal::new(String::new());
    let old_passwd = RwSignal::new(String::new());
    let passwd = RwSignal::new(String::new());
    let passwd_conf = RwSignal::new(String::new());
    let passwd_del = RwSignal::new(String::new());

    let id = move || cookie.get().unwrap_or_default();
    let on_submit_uname = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if update_username(id(), username.get()).await.is_ok() {
                username.set(String::new());
            }
        });
    };
    let on_submit_email = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if update_email(id(), email.get()).await.is_ok() {
                email.set(String::new());
            }
        });
    };
    let on_submit_bio = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if update_bio(id(), bio.get()).await.is_ok() {
                bio.set(String::new());
            }
        });
    };
    let on_submit_birth_date = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if update_birth_date(id(), birth_date.get()).await.is_ok() {
                birth_date.set(String::new());
            }
        });
    };
    let on_submit_change_passwd = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if change_password(id(), old_passwd.get(), passwd.get(), passwd_conf.get())
                .await
                .is_ok()
            {
                passwd.set(String::new());
                passwd_conf.set(String::new());
            }
            old_passwd.set(String::new());
        });
    };
    let on_submit_del_account = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if login_by_id(id(), passwd_del.get()).await.is_ok() {
                let _ = delete_user(id()).await;
                set_cookie.set(None);
                let _ = redirect_to_login().await;
            } else {
                passwd_del.set(String::new());
            }
        });
    };

    view! {
        <Title text="Edit Account"/>
        <h1>"Edit Account"</h1>
        <form on:submit=on_submit_uname>
            <div>
                <label for="username">"Username:"</label>
                <input
                    id="username"
                    type="text"
                    bind:value=username
                />
            </div>
            <div>
                <button type="submit">"Save"</button>
            </div>
        </form>
        <form on:submit=on_submit_email>
            <div>
                <label for="email">"Email:"</label>
                <input
                    id="email"
                    type="email"
                    bind:value=email
                />
            </div>
            <div>
                <button type="submit">"Save"</button>
            </div>
        </form>
        <form on:submit=on_submit_bio>
            <div>
                <label for="bio">"Bio:"</label>
                <input
                    type="text"
                    id="bio"
                    bind:value=bio
                />
            </div>
            <div>
                <button type="submit">"Save"</button>
            </div>
        </form>
        <form on:submit=on_submit_birth_date>
            <div>
                <label for="birth_date">"Birth Date:"</label>
                <input
                    type="date"
                    id="birth_date"
                    bind:value=birth_date
                />
            </div>
            <div>
                <button type="submit">"Save"</button>
            </div>
        </form>

        <h2>"Change Password"</h2>
        <form on:submit=on_submit_change_passwd>
            <div>
                <label for="old_passwd">"Old Password:"</label>
                <input
                    id="old_passwd"
                    type="password"
                    bind:value=old_passwd
                />
            </div>
            <div>
                <label for="passwd">"Password:"</label>
                <input
                    id="passwd"
                    type="password"
                    bind:value=passwd
                />
            </div>
            <div>
                <label for="passwd_conf">"Confirm Password:"</label>
                <input
                    id="passwd_conf"
                    type="password"
                    bind:value=passwd_conf
                />
            </div>
            <div>
                <button type="submit">"Save"</button>
            </div>
        </form>

        <h2>"Delete Account"</h2>
        <form on:submit=on_submit_del_account>
            <div>
                <label for="passwd_del">"Password:"</label>
                <input
                    id="passwd_del"
                    type="password"
                    bind:value=passwd_del
                />
            </div>
            <div>
                <button type="submit">"Confirm Account Deletion"</button>
            </div>
        </form>
    }
}
