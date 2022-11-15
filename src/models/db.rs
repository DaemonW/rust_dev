use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::{env, time};
use tracing::error;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
// pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type SqlErrKind = diesel::result::DatabaseErrorKind;
pub type SqlErr = diesel::result::Error;

pub fn init_conn_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        error!("DATABASE_URL environment variable not set!");
        std::process::exit(-1);
    });
    let manager = ConnectionManager::new(&db_url);
    Pool::builder()
        .max_size(1)
        .connection_timeout(time::Duration::from_secs(3))
        .test_on_check_out(true)
        .build(manager)
        .unwrap_or_else(|_| {
            error!("connect to db {} failed", &db_url);
            std::process::exit(-1);
        })
}
