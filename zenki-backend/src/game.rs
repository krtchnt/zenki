use std::fmt::Display;

use time::PrimitiveDateTime;
use zenki_util::usize_to_i32;

use crate::{Developer, State};

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "rating_n", rename_all = "snake_case")]
pub enum GameRating {
    General,
    Mature,
    Sensitive,
}

impl Display for GameRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::General => "General",
            Self::Mature => "Mature",
            Self::Sensitive => "Sensitive",
        })
    }
}

pub struct Game {
    pub gid: i32,
    pub gname: String,
    pub descr: Option<String>,
    pub rating: GameRating,
    pub release_at: Option<PrimitiveDateTime>,
    pub created_at: Option<PrimitiveDateTime>,
}

pub struct GameRef {
    pub gid: i32,
    pub gname: String,
}

pub enum WishlistStatus {
    NotInWishlist,
    InWishlist,
    Owned,
}

impl State {
    /// # Errors
    /// when querying the database failed
    pub async fn query_all_games(&self) -> sqlx::Result<Vec<Game>> {
        sqlx::query_as!(
            Game,
            r#"SELECT gid, gname, descr, rating AS "rating: _", release_at, created_at FROM games"#
        )
        .fetch_all(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_game(&self, id: usize) -> sqlx::Result<Option<Game>> {
        sqlx::query_as!(
            Game,
            r#"SELECT gid, gname, descr, rating AS "rating: _", release_at, created_at
            FROM games
            WHERE gid = $1"#,
            usize_to_i32(id)
        )
        .fetch_optional(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn add_game_to_wishlist(&self, uid: usize, gid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO game_user (uid, gid, wishlist) VALUES ($1, $2, TRUE);"#,
            usize_to_i32(uid),
            usize_to_i32(gid)
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn add_game_to_library(&self, uid: usize, gid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO game_user (uid, gid, wishlist) VALUES ($1, $2, FALSE);"#,
            usize_to_i32(uid),
            usize_to_i32(gid)
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn remove_game_from_wishlist(&self, uid: usize, gid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r#"DELETE FROM game_user WHERE gid = $1 AND uid = $2 AND wishlist = TRUE;"#,
            usize_to_i32(gid),
            usize_to_i32(uid),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_wishlist(&self, id: usize) -> sqlx::Result<Vec<Game>> {
        sqlx::query_as!(
            Game,
            r#"SELECT g.gid, gname, descr, rating AS "rating: _", release_at, created_at
            FROM games g
            JOIN game_user gu ON g.gid = gu.gid
            WHERE gu.uid = $1 AND gu.wishlist = TRUE;"#,
            usize_to_i32(id)
        )
        .fetch_all(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_library(&self, id: usize) -> sqlx::Result<Vec<Game>> {
        sqlx::query_as!(
            Game,
            r#"SELECT g.gid, gname, descr, rating AS "rating: _", release_at, created_at
            FROM games g
            JOIN game_user gu ON g.gid = gu.gid
            WHERE gu.uid = $1 AND gu.wishlist = FALSE;"#,
            usize_to_i32(id)
        )
        .fetch_all(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn get_wishlist_status(
        &self,
        uid: usize,
        gid: usize,
    ) -> sqlx::Result<WishlistStatus> {
        Ok(
            match sqlx::query!(
                r#"SELECT wishlist
                FROM game_user
                WHERE uid = $1 AND gid = $2;"#,
                usize_to_i32(uid),
                usize_to_i32(gid),
            )
            .fetch_optional(&self.db)
            .await?
            .map(|x| x.wishlist)
            {
                Some(Some(false) | None) => WishlistStatus::Owned,
                Some(Some(true)) => WishlistStatus::InWishlist,
                None => WishlistStatus::NotInWishlist,
            },
        )
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_other_games_from_same_developers(
        &self,
        gid: usize,
    ) -> sqlx::Result<Vec<GameRef>> {
        sqlx::query_as!(
            GameRef,
            r#"SELECT DISTINCT g.gid, g.gname
            FROM games g
            JOIN Developer_Game dg ON g.gid = dg.gid
            WHERE dg.did IN (
            SELECT did
            FROM Developer_Game
            WHERE gid = $1)
            AND g.gid <> $1;"#,
            usize_to_i32(gid)
        )
        .fetch_all(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_developers_by_game(&self, gid: usize) -> sqlx::Result<Vec<Developer>> {
        sqlx::query_as!(
            Developer,
            r#"SELECT d.did, d.dname, d.descr, d.created_at
            FROM developers d
            JOIN developer_game dg ON d.did = dg.did
            WHERE dg.gid = $1;"#,
            usize_to_i32(gid)
        )
        .fetch_all(&self.db)
        .await
    }
}
