mod activity;
pub mod app;
mod auth;
mod developer;
mod friendship;
mod game;
mod item;
mod page;
mod review;
mod route;
mod tag;
mod transaction;
mod user;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
