use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

use crate::config::CONFIG;

// #[cfg(feature = "postgres")]
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
// #[cfg(feature = "postgres")]
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;


pub fn init_pool(database_url:&str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("database_url error")
}

pub fn config_pool() -> DbPool {
    init_pool(&CONFIG.database_url)
}


