use self::super::schema::datasets::dsl::{created_on, data, datasets, id, in_use};
use crate::entities::datasets::DatasetInfo;
use crate::errors::dberror::DbError;
use crate::errors::dberror::DbErrorKind::{Connection, ReadFailed};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{r2d2, PgConnection};
use std::env;
use std::sync::Arc;

pub struct DatasetRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

pub type DatasetRepositoryArc = Arc<DatasetRepository>;

impl DatasetRepository {
    pub fn new() -> DatasetRepositoryArc {
        // See: https://stackoverflow.com/q/68633531/12347616
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create database pool");
        // let conn = PgConnection::establish(&database_url)
        //     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Arc::from(DatasetRepository { pool })
    }

    //noinspection RsUnresolvedReference
    pub fn datasets(&self) -> Result<Vec<DatasetInfo>, DbError> {
        let mut conn = self.connection()?;
        datasets
            .select((id, in_use, created_on))
            .load::<DatasetInfo>(&mut conn)
            .map_err(|e| DbError::source(ReadFailed, "No connection", e))
    }

    // fn connection(&self) -> Result<impl Connection, r2d2::PoolError> {
    fn connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, DbError> {
        self.pool
            .get()
            .map_err(|e| DbError::source(Connection, "No connection", e))
    }
}
