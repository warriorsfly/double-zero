// #[cfg(feature = "postgres")]
pub type Database = Pool<ConnectionManager<PgConnection>>;
// #[cfg(feature = "postgres")]
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn config_database(cfg: &mut ServiceConfig) {
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    let database = Pool::builder().build(manager).expect("database_url error");
    cfg.app_data(Data::new(database));
}