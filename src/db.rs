use mysql::{Opts, OptsBuilder};
use once_cell::sync::OnceCell;
use r2d2::{Pool, PooledConnection};
use r2d2_mysql::MySqlConnectionManager;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2_sqlite::SqliteConnectionManager;

static GLOBAL_PG_POOL: OnceCell<Pool<PostgresConnectionManager<NoTls>>> = OnceCell::new();
static GLOBAL_MY_POOL: OnceCell<Pool<MySqlConnectionManager>> = OnceCell::new();
static GLOBAL_SQLITE_POOL: OnceCell<Pool<SqliteConnectionManager>> = OnceCell::new();

pub struct Postgres {}
impl Postgres {
    pub fn init(url: &str) {
        let manager = PostgresConnectionManager::new(url.parse().unwrap(), NoTls);
        let pool = r2d2::Pool::new(manager).unwrap();
        GLOBAL_PG_POOL.set(pool).unwrap();
    }
    pub fn get() -> PooledConnection<PostgresConnectionManager<NoTls>> {
        let pool = GLOBAL_PG_POOL.get().unwrap().clone();
        pool.get().unwrap()
    }
}

pub struct MySQL {}
impl MySQL {
    pub fn init(url: &str) {
        let opts = Opts::from_url(url).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MySqlConnectionManager::new(builder);
        let pool = r2d2::Pool::new(manager).unwrap();
        GLOBAL_MY_POOL.set(pool).unwrap();
    }
    pub fn get() -> PooledConnection<MySqlConnectionManager> {
        let pool = GLOBAL_MY_POOL.get().unwrap().clone();
        pool.get().unwrap()
    }
}

pub struct SQLite {}
impl SQLite {
    pub fn init(url: &str) {
        let manager = SqliteConnectionManager::file(url);
        let pool = r2d2::Pool::new(manager).unwrap();
        GLOBAL_SQLITE_POOL.set(pool).unwrap();
    }
    pub fn get() -> PooledConnection<SqliteConnectionManager> {
        let pool = GLOBAL_SQLITE_POOL.get().unwrap().clone();
        pool.get().unwrap()
    }
}
