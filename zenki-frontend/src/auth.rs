use codee::string::FromToStringCodec;
use leptos::{prelude::*, server};

const LOGIN_SESSION_NAME: &str = "zenki-login-session";

#[inline]
pub fn get_login_session() -> (Signal<Option<usize>>, WriteSignal<Option<usize>>) {
    leptos_use::use_cookie::<_, FromToStringCodec>(LOGIN_SESSION_NAME)
}

#[server]
#[allow(clippy::unused_async)]
pub async fn is_logged_in() -> Result<bool, ServerFnError> {
    Ok(get_login_session().0.get().is_some())
}

#[server]
#[allow(clippy::unused_async)]
pub async fn set_logged_in(uid: usize) -> Result<(), ServerFnError> {
    get_login_session().1.set(Some(uid));
    Ok(())
}

#[server]
pub async fn login(username: String, password: String) -> Result<usize, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    if let Ok(Some(id)) = state.verify_password(&username, &password).await {
        Ok(id)
    } else {
        Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        ))
    }
}

#[server]
pub async fn login_by_id(uid: usize, password: String) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    if matches!(state.verify_password_by_id(uid, &password).await, Ok(true)) {
        Ok(())
    } else {
        Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        ))
    }
}

#[server]
pub async fn register(
    username: String,
    password: String,
    password_confirm: String,
) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    if password == password_confirm && state.register(&username, &password).await.is_ok() {
        crate::route::redirect_to_login().await
    } else {
        Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        ))
    }
}

#[server]
pub async fn change_password(
    id: usize,
    old_passwd: String,
    passwd: String,
    passwd_conf: String,
) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    if passwd != passwd_conf
        || state
            .change_password(id, &old_passwd, &passwd)
            .await
            .is_err()
    {
        return Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        ));
    }
    Ok(())
}
