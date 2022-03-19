use diesel::{r2d2::{Pool, PooledConnection, ConnectionManager}, PgConnection};

use crate::config::CONFIG;

// #[cfg(feature = "postgres")]
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
// #[cfg(feature = "postgres")]
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    Pool::builder().build(manager).expect("database_url error")
}
