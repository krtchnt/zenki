use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct GameActivity {
    pub uid: usize,
    pub uname: String,
    pub gid: usize,
    pub gname: String,
    pub startplay_at: String,
    pub duration: Option<String>,
}

#[cfg(feature = "ssr")]
impl From<zenki_backend::GameActivity> for GameActivity {
    fn from(value: zenki_backend::GameActivity) -> Self {
        use zenki_util::i32_to_usize;
        Self {
            uid: i32_to_usize(value.uid),
            uname: value.uname,
            gid: i32_to_usize(value.gid),
            gname: value.gname,
            startplay_at: value.startplay_at.to_string(),
            duration: value
                .duration
                .map(|x| zenki_backend::pg_interval_to_time_duration(x).to_string()),
        }
    }
}

#[server]
pub async fn start_playing(uid: usize, gid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.start_playing(uid, gid).await?)
}
#[server]
pub async fn stop_playing(uid: usize, gid: usize) -> Result<(), ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.stop_playing(uid, gid).await?)
}
#[server]
pub async fn is_playing(uid: usize, gid: usize) -> Result<bool, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state.is_playing(uid, gid).await?)
}

#[server]
pub async fn get_game_activity(uid: usize) -> Result<Vec<GameActivity>, ServerFnError> {
    let state = expect_context::<zenki_backend::State>();
    Ok(state
        .query_game_activity(uid)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}
