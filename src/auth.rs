use codee::string::FromToStringCodec;
use leptos::{
    prelude::{ServerFnError, Signal, WriteSignal},
    server,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Credentials {
    username: String,
    password: String,
}

const LOGIN_SESSION_NAME: &str = "zenki-login-session";

#[inline]
pub fn get_login_session() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    leptos_use::use_cookie::<_, FromToStringCodec>(LOGIN_SESSION_NAME)
}

#[server]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    // pull the DB pool and auth provider from context
    //let pool = pool()?;
    //let auth = auth()?;

    // check whether the user exists
    //let user: User = User::get_from_username(username, &pool)
    //    .await
    //    .ok_or_else(|| ServerFnError::ServerError("User does not exist.".into()))?;

    // check whether the user has provided the correct password
    //match verify(password, &user.password)? {
    match password == "password" {
        // if the password is correct...
        true => {
            // log the user in
            //auth.login_user(user.id);
            //auth.remember_user(remember.is_some());

            // and redirect to the home page
            leptos_axum::redirect("/app");
            Ok(())
        }
        // if not, return an error
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server]
async fn authenticate_user(credentials: Credentials) -> Result<bool, ServerFnError> {
    // Replace with your authentication logic
    Ok(credentials.username == "admin" && credentials.password == "password")
}

#[server]
pub async fn register(
    username: String,
    password: String,
    password_confirm: String,
) -> Result<(), ServerFnError> {
    // pull the DB pool and auth provider from context
    //let pool = pool()?;
    //let auth = auth()?;

    // check whether the user exists
    //let user: User = User::get_from_username(username, &pool)
    //    .await
    //    .ok_or_else(|| ServerFnError::ServerError("User does not exist.".into()))?;

    // check whether the user has provided the correct password
    //match verify(password, &user.password)? {
    match password == password_confirm {
        // if the password is correct...
        true => {
            // log the user in
            //auth.login_user(user.id);
            //auth.remember_user(remember.is_some());

            // and redirect to the home page
            leptos_axum::redirect("/login");
            Ok(())
        }
        // if not, return an error
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}
