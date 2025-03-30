use std::{fmt::Display, str::FromStr};

use thiserror::Error;
use time::PrimitiveDateTime;
use zenki_util::{i32_to_usize, usize_to_i32};

use crate::{PurchaseType, State};

#[derive(Error, Debug)]
#[error("error while parsing payment method via string")]
pub struct ParsePaymentMethodError;

#[derive(sqlx::Type)]
#[sqlx(type_name = "payment_n", rename_all = "snake_case")]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    Paypal,
    Etc,
}

impl Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CreditCard => "Credit Card",
            Self::DebitCard => "Debit Card",
            Self::Paypal => "PayPal",
            Self::Etc => "etc.",
        })
    }
}

impl FromStr for PaymentMethod {
    type Err = ParsePaymentMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "credit_card" => Ok(Self::CreditCard),
            "debit_card" => Ok(Self::DebitCard),
            "paypal" => Ok(Self::Paypal),
            "etc" => Ok(Self::Etc),
            _ => Err(ParsePaymentMethodError),
        }
    }
}

pub struct TransactionHistory {
    pub tid: i32,
    pub gid: i32,
    pub gname: String,
    pub pid: i32,
    pub p_descr: Option<String>,
    pub bought_at: Option<PrimitiveDateTime>,
}

pub struct RichTransaction {
    pub tid: i32,
    pub uid: i32,
    pub receiver_uid: Option<i32>,
    pub pid: i32,
    pub payment_method: PaymentMethod,
    pub amount: i32,
    pub bought_at: Option<PrimitiveDateTime>,
    pub status: Option<String>,
    pub s_uname: String,
    pub r_uname: String,
    pub p_descr: Option<String>,
}

impl State {
    /// # Errors
    /// when querying the database failed
    pub async fn query_rich_transaction(
        &self,
        tid: usize,
    ) -> sqlx::Result<Option<RichTransaction>> {
        sqlx::query_as!(
            RichTransaction,
            r#"SELECT
                t.tid,
                t.uid,
                t.receiver_uid,
                t.pid,
                t.payment_method AS "payment_method: _",
                t.amount,
                t.bought_at,
                t.status,
                p.descr AS "p_descr",
                sender.uname AS "s_uname",
                receiver.uname AS "r_uname"
            FROM transactions t
            LEFT JOIN purchases p ON t.pid = p.pid
            LEFT JOIN users sender ON t.uid = sender.uid
            LEFT JOIN users receiver ON t.receiver_uid = receiver.uid
            WHERE t.tid = $1"#,
            usize_to_i32(tid)
        )
        .fetch_optional(&self.db)
        .await
    }

    /// # Errors
    /// when querying the database failed
    pub async fn create_transaction(
        &self,
        uid: usize,
        pid: usize,
        ruid: usize,
        payment_method: PaymentMethod,
        amount: usize,
    ) -> sqlx::Result<()> {
        let purchase = self.expect_purchase(pid).await?;
        if matches!(purchase.purchase_type, PurchaseType::GamePurchase) {
            let gid = i32_to_usize(purchase.gid);
            self.remove_game_from_wishlist(ruid, gid).await?;
            self.add_game_to_library(ruid, gid).await?;
        }

        sqlx::query!(
            r#"INSERT INTO transactions (uid, pid, receiver_uid, payment_method, amount)
            VALUES ($1, $2, $3, $4, $5)"#,
            usize_to_i32(uid),
            usize_to_i32(pid),
            usize_to_i32(ruid),
            payment_method as PaymentMethod,
            usize_to_i32(amount),
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    /// # Errors
    /// when querying the database failed
    pub async fn query_transaction_history(
        &self,
        uid: usize,
    ) -> sqlx::Result<Vec<TransactionHistory>> {
        sqlx::query_as!(
            TransactionHistory,
            r#"SELECT
            t.tid,
            g.gid,
            g.gname,
            p.pid,
            p.descr AS p_descr,
            t.bought_at
            FROM transactions t
            JOIN purchases p ON t.pid = p.pid
            JOIN games g ON p.gid = g.gid
            WHERE t.uid = $1
            ORDER BY t.bought_at DESC;"#,
            usize_to_i32(uid)
        )
        .fetch_all(&self.db)
        .await
    }
}
