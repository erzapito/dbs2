//! Database-related functions
use crate::config::{Config, CONFIG};
use actix_web::web;
use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, PoolError},
    sqlite::SqliteConnection,
    Connection,
};

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum DatabaseConnection {
    Mysql,
    Sqlite,
}

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type MysqlPool = Pool<MysqlConnection>;
pub type SqlitePool = Pool<SqliteConnection>;

#[cfg(all(feature = "mysql",not(test)))]
pub type PoolType = MysqlPool;

#[cfg(any(feature = "sqlite",test))]
pub type PoolType = SqlitePool;

#[derive(Clone)]
pub enum InferPool {
    Mysql(MysqlPool),
    Sqlite(SqlitePool),
}

impl InferPool {
    pub fn init_pool(config: Config) -> Result<Self, r2d2::Error> {
        match config.database {
            DatabaseConnection::Mysql => init_pool::<MysqlConnection>(config).map(InferPool::Mysql),
            DatabaseConnection::Sqlite => {
                init_pool::<SqliteConnection>(config).map(InferPool::Sqlite)
            }
        }
        .map_err(Into::into)
    }
}

pub fn init_pool<T>(config: Config) -> Result<Pool<T>, PoolError>
where
    T: Connection + 'static,
{
    let manager = ConnectionManager::<T>::new(config.database_url);
    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = InferPool::init_pool(CONFIG.clone()).expect("Failed to create connection pool");
    match pool {
        InferPool::Mysql(mysql_pool) => cfg.data(mysql_pool),
        InferPool::Sqlite(sqlite_pool) => cfg.data(sqlite_pool),
    };
}
