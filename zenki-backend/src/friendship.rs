use zenki_util::usize_to_i32;

use crate::{State, user::User};

pub enum FriendshipStatus {
    NotFriends,
    RequestSent,
    RequestReceived,
    Friends,
}

impl State {
    /// # Errors
    /// when querying the database failed
    pub async fn get_friendship_status(
        &self,
        uid: usize,
        fid: usize,
    ) -> sqlx::Result<FriendshipStatus> {
        let rec = sqlx::query!(
            r#"SELECT uid, pending FROM friends WHERE (uid = $1 AND fid = $2) OR (uid = $2 AND fid = $1)"#,
            usize_to_i32(uid),
            usize_to_i32(fid),
        )
        .fetch_all(&self.db)
        .await?;

        Ok(match rec.as_slice() {
            [] => FriendshipStatus::NotFriends,
            [row] if row.uid == usize_to_i32(uid) && row.pending => FriendshipStatus::RequestSent,
            [row] if row.uid == usize_to_i32(fid) && row.pending => {
                FriendshipStatus::RequestReceived
            }
            _ => FriendshipStatus::Friends,
        })
    }

    /// # Errors
    /// when querying the database failed
    pub async fn send_friend_request(&self, uid: usize, fid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r"INSERT INTO friends (uid, fid, pending) VALUES ($1, $2, TRUE);",
            usize_to_i32(uid),
            usize_to_i32(fid)
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn accept_friend_request(&self, uid: usize, fid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r"UPDATE friends SET pending = FALSE, added_at = NOW() WHERE uid = $1 AND fid = $2",
            usize_to_i32(fid),
            usize_to_i32(uid),
        )
        .execute(&self.db)
        .await?;

        sqlx::query!(
            r"INSERT INTO friends (uid, fid, pending) VALUES ($1, $2, FALSE);",
            usize_to_i32(uid),
            usize_to_i32(fid),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn decline_friend_request(&self, uid: usize, fid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r#"DELETE FROM friends WHERE uid = $1 AND fid = $2 AND pending = TRUE"#,
            usize_to_i32(fid),
            usize_to_i32(uid),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn cancel_friend_request(&self, uid: usize, fid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r#"DELETE FROM friends WHERE uid = $1 AND fid = $2 AND pending = TRUE"#,
            usize_to_i32(uid),
            usize_to_i32(fid),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn remove_friend(&self, uid: usize, fid: usize) -> sqlx::Result<()> {
        sqlx::query!(
            r#"DELETE FROM friends WHERE (uid = $1 AND fid = $2 OR uid = $2 AND fid = $1) AND pending = FALSE"#,
            usize_to_i32(uid),
            usize_to_i32(fid),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_friends(&self, uid: usize) -> sqlx::Result<Vec<User>> {
        sqlx::query_as!(
            User,
            r"SELECT users.* FROM friends INNER JOIN users ON friends.fid = users.uid WHERE friends.uid = $1 AND NOT friends.pending",
            usize_to_i32(uid)
        )
        .fetch_all(&self.db)
        .await
    }
}
