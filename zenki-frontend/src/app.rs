#![allow(clippy::must_use_candidate)]
use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    ParamSegment, SsrMode, StaticSegment,
    components::{FlatRoutes, ProtectedRoute, Route, Router},
};

use crate::{
    auth::{SetLoggedIn, is_logged_in},
    page::{Account, Developer, Game, Home, Item, Login, Main, Register, Tag, Transaction, User},
    route::{ACCOUNT, DEVELOPER, GAME, HOME, ITEM, LOGIN, MAIN, REGISTER, TAG, TRANSACTION, USER},
};

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();
    let set_logged_in_act = ServerAction::<SetLoggedIn>::new();
    let is_admin = Resource::new(
        move || set_logged_in_act.version().get(),
        |_| is_logged_in(),
    );

    view! {
        <Stylesheet id="leptos" href="/pkg/ssr_modes.css"/>
        <Title text="Welcome to Leptos"/>
        <Meta name="color-scheme" content="dark light"/>
        <Router>
            <nav>
                <a href=HOME>"Home"</a>
                "|"
                <a href=MAIN>"Main"</a>
            </nav>
            <main>
                <FlatRoutes fallback>
                    // We’ll load the home page with out-of-order streaming and <Suspense/>
                    <Route path=StaticSegment(HOME) view=Home/>
                    <ProtectedRoute
                        path=StaticSegment(LOGIN)
                        view=Login
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| !(n.unwrap_or(false)))
                        redirect_path=|| MAIN
                    />
                    <ProtectedRoute
                        path=StaticSegment(REGISTER)
                        view=Register
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| !(n.unwrap_or(false)))
                        redirect_path=|| MAIN
                    />

                    // We'll load the users with async rendering, so they can set
                    // the title and metadata *after* loading the data
                    <ProtectedRoute
                        path=StaticSegment(MAIN)
                        view=Main
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| n.unwrap_or(false))
                        redirect_path=|| LOGIN
                    />
                    <ProtectedRoute
                        path=StaticSegment(ACCOUNT)
                        view=Account
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| n.unwrap_or(false))
                        redirect_path=|| LOGIN
                    />
                    <ProtectedRoute
                        path=(StaticSegment(USER), ParamSegment("id"))
                        view=User
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| n.unwrap_or(false))
                        redirect_path=|| LOGIN
                    />
                    <ProtectedRoute
                        path=(StaticSegment(GAME), ParamSegment("id"))
                        view=Game
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| n.unwrap_or(false))
                        redirect_path=|| LOGIN
                    />
                    <ProtectedRoute
                        path=(StaticSegment(TAG), ParamSegment("tname"))
                        view=Tag
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| n.unwrap_or(false))
                        redirect_path=|| LOGIN
                    />
                    <ProtectedRoute
                        path=(StaticSegment(ITEM), ParamSegment("id"))
                        view=Item
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| n.unwrap_or(false))
                        redirect_path=|| LOGIN
                    />
                    <ProtectedRoute
                        path=(StaticSegment(DEVELOPER), ParamSegment("id"))
                        view=Developer
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| n.unwrap_or(false))
                        redirect_path=|| LOGIN
                    />
                    <ProtectedRoute
                        path=(StaticSegment(TRANSACTION), ParamSegment("id"))
                        view=Transaction
                        ssr=SsrMode::Async
                        condition=move || is_admin.get().map(|n| n.unwrap_or(false))
                        redirect_path=|| LOGIN
                    />
                </FlatRoutes>
            </main>
        </Router>
    }
}
