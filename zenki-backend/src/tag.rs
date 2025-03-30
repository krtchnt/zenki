use zenki_util::usize_to_i32;

use crate::{Game, State};

pub struct Tag {
    pub tname: String,
    pub descr: Option<String>,
    pub category: Option<String>,
}

impl State {
    /// # Errors
    /// when querying the database failed
    pub async fn query_tag(&self, tname: String) -> sqlx::Result<Option<Tag>> {
        sqlx::query_as!(Tag, r#"SELECT * FROM tags WHERE tname = $1"#, tname)
            .fetch_optional(&self.db)
            .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_tags(&self, gid: usize) -> sqlx::Result<Vec<Tag>> {
        sqlx::query_as!(
            Tag,
            r#"SELECT t.tname, t.descr, t.category FROM game_tag gt JOIN tags t ON t.tname = gt.tname WHERE gt.gid = $1"#,
            usize_to_i32(gid)
        )
        .fetch_all(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_games_with_tag(&self, tname: String) -> sqlx::Result<Vec<Game>> {
        sqlx::query_as!(
            Game,
            r#"SELECT g.gid, g.gname, g.descr, g.rating AS "rating: _", g.release_at, g.created_at
            FROM games g JOIN game_tag gt ON gt.gid = g.gid
            WHERE gt.tname = $1"#,
            tname
        )
        .fetch_all(&self.db)
        .await
    }
}
