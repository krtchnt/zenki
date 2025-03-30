use sqlx::postgres::types::PgInterval;
use time::{Duration, PrimitiveDateTime};
use zenki_util::usize_to_i32;

use crate::State;

pub struct GameActivity {
    pub uid: i32,
    pub uname: String,
    pub gid: i32,
    pub gname: String,
    pub startplay_at: PrimitiveDateTime,
    pub duration: Option<PgInterval>,
}

impl State {
    /// # Errors
    /// when connecting to the database failed
    pub async fn start_playing(&self, uid: usize, gid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO game_interaction (uid, gid, startplay_at)
            VALUES ($1, $2, NOW())"#,
            usize_to_i32(uid),
            usize_to_i32(gid),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when connecting to the database failed
    pub async fn stop_playing(&self, uid: usize, gid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r#"WITH latest_interaction AS (
                SELECT startplay_at
                FROM game_interaction
                WHERE uid = $1 AND gid = $2 AND duration IS NULL
                ORDER BY startplay_at DESC
                LIMIT 1
            )
            UPDATE game_interaction
            SET duration = NOW() - li.startplay_at
            FROM latest_interaction li
            WHERE game_interaction.uid = $1 AND game_interaction.gid = $2
            AND game_interaction.startplay_at = li.startplay_at
            AND game_interaction.duration IS NULL"#,
            usize_to_i32(uid),
            usize_to_i32(gid),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when connecting to the database failed
    pub async fn is_playing(&self, uid: usize, gid: usize) -> sqlx::Result<bool> {
        let row = sqlx::query!(
            r#"SELECT duration
             FROM game_interaction
             WHERE uid = $1 AND gid = $2
             ORDER BY startplay_at DESC
             LIMIT 1"#,
            usize_to_i32(uid),
            usize_to_i32(gid),
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(row.is_some_and(|x| x.duration.is_none()))
    }

    /// # Errors
    /// when connecting to the database failed
    pub async fn query_game_activity(&self, uid: usize) -> sqlx::Result<Vec<GameActivity>> {
        sqlx::query_as!(
            GameActivity,
            r#"SELECT
                gi.uid,
                u.uname,
                gi.gid,
                g.gname,
                gi.startplay_at,
                gi.duration
            FROM
                game_interaction gi
            JOIN
                users u ON gi.uid = u.uid
            JOIN
                games g ON gi.gid = g.gid
            WHERE
                gi.uid = $1
            ORDER BY
                gi.startplay_at DESC;"#,
            usize_to_i32(uid),
        )
        .fetch_all(&self.db)
        .await
    }
}

#[must_use]
pub const fn pg_interval_to_time_duration(x: PgInterval) -> Duration {
    Duration::microseconds(x.microseconds)
}
