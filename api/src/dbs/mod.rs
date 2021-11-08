use diesel::pg::PgConnection;
use diesel::r2d2::{
    Builder,
    ConnectionManager,
    Pool,
};
use diesel::prelude::*;
use crate::utils::G;
use crate::constants::{DATABASE_URL};

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    let url = G.get(DATABASE_URL).unwrap();
    PgConnection::establish(url)
        .expect(&format!("Error connecting to {}", url))
}

pub fn get_connection_pool() -> ConnectionPool {
    let url = G.get(DATABASE_URL).unwrap();
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(url);
    Builder::new()
        .max_size(10)
        .min_idle(Some(10))
        .build(manager)
        .expect("Connection pool initialization failed")
}