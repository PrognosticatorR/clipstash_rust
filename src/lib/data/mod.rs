pub mod model;
pub mod query;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite, sqlite::SqlitePool, Sqlite};
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

type AppDatabase = Database<Sqlite>;
pub type DatabasePool = SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlite::SqliteRow;
pub type AppQueryResult = sqlite::SqliteQueryResult;

struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!(
                    "If the data base is not yet been created, run:  \n $sqlx database setup \n"
                );
                panic!("database error")
            }
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Debug, Clone, Display, From, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}
impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}
