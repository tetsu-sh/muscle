use crate::constants::env_key;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;

use super::errors::MyError;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var(env_key::DATABASE_URL).expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

pub async fn establish_sqlx_connection() -> MySqlPool {
    dotenv().ok();
    let database_url = env::var(env_key::DATABASE_URL).expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("db connection error");
    pool
}
