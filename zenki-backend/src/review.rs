use time::PrimitiveDateTime;
use zenki_util::usize_to_i32;

use crate::State;

pub struct Review {
    pub rid: i32,
    pub uid: i32,
    pub uname: String,
    pub rated: Option<f64>,
    pub reviewed_text: Option<String>,
    pub reviewed_at: Option<PrimitiveDateTime>,
}

impl State {
    /// # Errors
    /// when querying the database failed
    pub async fn query_reviews(&self, id: usize) -> sqlx::Result<Vec<Review>> {
        sqlx::query_as!(
            Review,
            r#"SELECT r.rid, r.uid, u.uname, r.rated, r.reviewed_text, r.reviewed_at
            FROM reviews r
            JOIN users u ON u.uid = r.uid
            WHERE gid = $1"#,
            usize_to_i32(id)
        )
        .fetch_all(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn post_review(
        &self,
        uid: usize,
        gid: usize,
        rated: f64,
        reviewed_text: String,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO reviews (gid, uid, rated, reviewed_text)
            VALUES ($1, $2, $3, $4);"#,
            usize_to_i32(gid),
            usize_to_i32(uid),
            rated,
            reviewed_text
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
}
