use self::super::schema::datasets::dsl::{created_on, datasets, id, in_use};
use crate::entities::datasets::DatasetInfo;
use crate::errors::db_error::DbError;
use crate::errors::db_error::DbErrorKind::{Connection, ReadFailed};
use crate::errors::std_error::StdError;
use crate::repositories::traits::DatasetRepository;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{r2d2, PgConnection};
use std::env;
use std::error::Error;


pub struct PostgresDatasetRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresDatasetRepository {
    pub fn new() -> Self {
        // See: https://stackoverflow.com/q/68633531/12347616
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create database pool");
        // let conn = PgConnection::establish(&database_url)
        //     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        PostgresDatasetRepository { pool }
    }

    // fn connection(&self) -> Result<impl Connection, r2d2::PoolError> {
    fn connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, DbError> {
        self.pool
            .get()
            .map_err(|e| StdError::from(e))
            .map_err(|e| DbError::source(Connection, "No connection", e))
    }
}

impl DatasetRepository for PostgresDatasetRepository {
    //noinspection RsUnresolvedReference
    fn datasets(&self) -> Result<Vec<DatasetInfo>, DbError> {
        datasets
            .select((id, in_use, created_on))
            .load::<DatasetInfo>(&mut self.connection()?)
            .map_err(|e| StdError::from(e))
            .map_err(|e| DbError::source(ReadFailed, "No connection", e))
    }
}
