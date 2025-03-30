mod activity;
mod auth;
mod developer;
mod friendship;
mod game;
mod purchase;
mod review;
mod tag;
mod transaction;
mod user;

use std::str::FromStr;

use argon2::Argon2;
use log::LevelFilter;
use sqlx::{
    ConnectOptions, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

pub use {
    activity::{GameActivity, pg_interval_to_time_duration},
    developer::Developer,
    friendship::FriendshipStatus,
    game::{Game, GameRef, WishlistStatus},
    purchase::{Purchase, PurchaseType},
    review::Review,
    tag::Tag,
    transaction::{RichTransaction, TransactionHistory},
    user::{User, parse_html_date},
};

#[derive(Clone)]
pub struct State {
    db: PgPool,
    argon2: Argon2<'static>,
}

impl State {
    /// # Errors
    /// when connecting to the database failed
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = dotenvy::var("DATABASE_URL")?;
        let options = PgConnectOptions::from_str(&database_url)?.log_statements(LevelFilter::Debug);
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        Ok(Self {
            db,
            argon2: Argon2::default(),
        })
    }
}
