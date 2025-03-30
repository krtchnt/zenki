use constcat::concat as const_concat;
use leptos::{prelude::ServerFnError, server};

pub const HOME: &str = "/";
pub const REGISTER: &str = const_concat!(HOME, "register");
pub const LOGIN: &str = const_concat!(HOME, "login");
pub const MAIN: &str = const_concat!(HOME, "app");
pub const USER: &str = const_concat!(HOME, "user");
pub const GAME: &str = const_concat!(HOME, "game");
pub const ACCOUNT: &str = const_concat!(HOME, "account");
pub const TAG: &str = const_concat!(HOME, "tag");
pub const ITEM: &str = const_concat!(HOME, "item");
pub const TRANSACTION: &str = const_concat!(HOME, "transaction");
pub const DEVELOPER: &str = const_concat!(HOME, "developer");

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
