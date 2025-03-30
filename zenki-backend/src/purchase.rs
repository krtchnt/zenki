use std::fmt::Display;

use time::PrimitiveDateTime;
use zenki_util::usize_to_i32;

use crate::State;

#[derive(sqlx::Type)]
#[sqlx(type_name = "purchase_n", rename_all = "snake_case")]
pub enum PurchaseType {
    GamePurchase,
    InGamePurchase,
    Subscriptions,
    Dlc,
    Etc,
}

impl Display for PurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::GamePurchase => "Game purchase",
            Self::InGamePurchase => "In-game purchase",
            Self::Subscriptions => "Subscription",
            Self::Dlc => "DLC",
            Self::Etc => "etc.",
        })
    }
}

pub struct Purchase {
    pub pid: i32,
    pub gid: i32,
    #[allow(clippy::struct_field_names)]
    pub purchase_type: PurchaseType,
    pub price: f64,
    pub descr: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
}

impl State {
    /// # Errors
    /// when querying the database failed
    pub async fn query_purchases(&self, gid: usize) -> sqlx::Result<Vec<Purchase>> {
        sqlx::query_as!(
            Purchase,
            r#"SELECT pid, gid, purchase_type AS "purchase_type: _", price, descr, created_at
            FROM purchases WHERE gid = $1"#,
            usize_to_i32(gid)
        )
        .fetch_all(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_purchase(&self, pid: usize) -> sqlx::Result<Option<Purchase>> {
        sqlx::query_as!(
            Purchase,
            r#"SELECT pid, gid, purchase_type AS "purchase_type: _", price, descr, created_at
            FROM purchases WHERE pid = $1"#,
            usize_to_i32(pid)
        )
        .fetch_optional(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn expect_purchase(&self, pid: usize) -> sqlx::Result<Purchase> {
        sqlx::query_as!(
            Purchase,
            r#"SELECT pid, gid, purchase_type AS "purchase_type: _", price, descr, created_at
            FROM purchases WHERE pid = $1"#,
            usize_to_i32(pid)
        )
        .fetch_one(&self.db)
        .await
    }
}
