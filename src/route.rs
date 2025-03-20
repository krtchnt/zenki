use leptos::{prelude::ServerFnError, server};

pub const HOME: &str = "/";
pub const REGISTER: &str = "/register";
pub const LOGIN: &str = "/login";
pub const MAIN: &str = "/app";

#[server]
#[allow(clippy::unused_async)]
pub async fn redirect_to_main() -> Result<(), ServerFnError> {
    leptos_axum::redirect(MAIN);
    Ok(())
}

#[server]
#[allow(clippy::unused_async)]
pub async fn redirect_to_register() -> Result<(), ServerFnError> {
    leptos_axum::redirect(REGISTER);
    Ok(())
}

#[server]
#[allow(clippy::unused_async)]
pub async fn redirect_to_login() -> Result<(), ServerFnError> {
    leptos_axum::redirect(LOGIN);
    Ok(())
}
