use crate::utils;
use crate::utils::errors::MyError;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use sqlx::MySqlPool;

type AppConn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct AppState {
    // pub pool: utils::db::DbPool,
    pub sqlx_db: sqlx::Pool<sqlx::MySql>,
}

impl AppState {
    // pub fn get_conn(&self) -> Result<AppConn, MyError> {
    //     let conn = self.pool.get()?;
    //     Ok(conn)
    // }
    pub fn get_sqls_db_conn(&self) -> Result<MySqlPool, MyError> {
        let conn = self.sqlx_db.clone();
        Ok(conn)
    }
}
