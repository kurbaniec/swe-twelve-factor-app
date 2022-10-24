use self::super::schema::datasets::dsl::{created_on, datasets, id, in_use};
use crate::entities::datasets::{DatasetInfo, DatasetInsert};
use crate::errors::db_error::DbError;
use crate::errors::db_error::DbErrorKind::{
    Connection as ConnFailed, CreateFailed, ReadFailed, UpdateFailed,
};
use crate::errors::std_error::StdError;
use crate::repositories::traits::DatasetRepository;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{insert_into, r2d2, update, Connection, PgConnection};
use std::env;
use std::error::Error;

pub struct PostgresDatasetRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

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
    fn connection(&self) -> Result<DbConnection, DbError> {
        self.pool
            .get()
            .map_err(StdError::from)
            .map_err(|e| DbError::source(ConnFailed, "No connection", e))
    }

    //noinspection RsUnresolvedReference
    fn add_dataset(
        &self,
        dataset: &DatasetInsert,
        conn: &mut DbConnection,
    ) -> Result<DatasetInfo, DbError> {
        insert_into(datasets)
            .values(dataset)
            .returning((id, in_use, created_on))
            .get_result::<DatasetInfo>(conn)
            .map_err(StdError::from)
            .map_err(|e| DbError::source(CreateFailed, "Could not save dataset", e))
    }

    //noinspection RsUnresolvedReference
    fn update_all_not_in_use(&self, conn: &mut DbConnection) -> Result<(), DbError> {
        update(datasets)
            .filter(in_use.eq(true))
            .set(in_use.eq(false))
            .execute(conn)
            .map(|_| ())
            .map_err(StdError::from)
            .map_err(|e| DbError::source(UpdateFailed, "Could not set in_use", e))
    }
}

impl DatasetRepository for PostgresDatasetRepository {
    //noinspection RsUnresolvedReference
    fn datasets(&self) -> Result<Vec<DatasetInfo>, DbError> {
        datasets
            .select((id, in_use, created_on))
            .load::<DatasetInfo>(&mut self.connection()?)
            .map_err(StdError::from)
            .map_err(|e| DbError::source(ReadFailed, "No connection", e))
    }

    fn add_dataset(&self, dataset: &DatasetInsert) -> Result<DatasetInfo, DbError> {
        let mut conn = self.connection()?;
        conn.transaction::<_, DbError, _>(|conn| {
            if dataset.in_use {
                self.update_all_not_in_use(conn)?;
            }
            let info = self.add_dataset(dataset, conn)?;
            Ok(info)
        })
    }
}
