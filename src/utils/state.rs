use crate::utils;
use crate::utils::errors::MyError;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};


type AppConn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: utils::db::DbPool,
}

impl AppState {
    pub fn get_conn(&self) -> Result<AppConn, MyError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}
