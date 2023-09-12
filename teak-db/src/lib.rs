pub mod error;
pub mod models;
pub mod schema;
pub mod tree;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;

pub use error::Error;

pub fn connect() -> Result<PgConnection, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|_| Error::NoDatabaseUrl)?;
    let connection = PgConnection::establish(&database_url)?;

    Ok(connection)
}

pub fn connection_pool() -> Result<Pool<ConnectionManager<PgConnection>>, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|_| Error::NoDatabaseUrl)?;
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder().build(manager)?;

    Ok(pool)
}
