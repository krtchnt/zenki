use time::PrimitiveDateTime;
use zenki_util::usize_to_i32;

use crate::{Game, State};

pub struct Developer {
    pub did: i32,
    pub dname: String,
    pub descr: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
}

impl State {
    /// # Errors
    /// when querying the database failed
    pub async fn query_developer(&self, did: usize) -> sqlx::Result<Option<Developer>> {
        sqlx::query_as!(
            Developer,
            r#"SELECT * FROM developers WHERE did = $1"#,
            usize_to_i32(did)
        )
        .fetch_optional(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_developers(&self) -> sqlx::Result<Vec<Developer>> {
        sqlx::query_as!(Developer, r#"SELECT * FROM developers"#,)
            .fetch_all(&self.db)
            .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_games_from_developer(&self, did: usize) -> sqlx::Result<Vec<Game>> {
        sqlx::query_as!(
            Game,
            r#"SELECT g.gid, g.gname, g.descr, g.rating AS "rating: _", g.release_at, g.created_at
            FROM games g
            JOIN developer_game dg ON g.gid = dg.gid
            JOIN developers d ON dg.did = d.did
            WHERE d.did = $1;"#,
            usize_to_i32(did)
        )
        .fetch_all(&self.db)
        .await
    }
}
