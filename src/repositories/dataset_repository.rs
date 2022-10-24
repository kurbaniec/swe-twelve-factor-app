use self::super::schema::datasets::dsl::{
    created_on as dsl_created_on, data as dsl_data, datasets as dsl_datasets, id as dsl_id,
    in_use as dsl_in_use,
};
use crate::entities::datasets::{DatasetInfo, DatasetInsert};
use crate::errors::db_error::DbError;
use crate::errors::db_error::DbErrorKind::{
    Connection as ConnFailed, CreateFailed, DeleteFailed, ReadFailed, UpdateFailed,
};
use crate::errors::std_error::StdError;
use crate::repositories::traits::DatasetRepository;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{delete, insert_into, r2d2, update, Connection, PgConnection};
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

    fn dataset(&self, id: i32, conn: &mut DbConnection) -> Result<DatasetInfo, DbError> {
        dsl_datasets
            .filter(dsl_id.eq(id))
            .select((dsl_id, dsl_in_use, dsl_created_on))
            .get_result::<DatasetInfo>(conn)
            .map_err(StdError::from)
            .map_err(|e| DbError::source(ReadFailed, "Dataset not found", e))
    }

    fn add_dataset(
        &self,
        dataset: &DatasetInsert,
        conn: &mut DbConnection,
    ) -> Result<DatasetInfo, DbError> {
        insert_into(dsl_datasets)
            .values(dataset)
            .returning((dsl_id, dsl_in_use, dsl_created_on))
            .get_result::<DatasetInfo>(conn)
            .map_err(StdError::from)
            .map_err(|e| DbError::source(CreateFailed, "Could not save dataset", e))
    }

    fn update_all_not_in_use(&self, conn: &mut DbConnection) -> Result<(), DbError> {
        update(dsl_datasets)
            .filter(dsl_in_use.eq(true))
            .set(dsl_in_use.eq(false))
            .execute(conn)
            .map(|_| ())
            .map_err(StdError::from)
            .map_err(|e| DbError::source(UpdateFailed, "Could not set in_use", e))
    }

    fn update_in_use(&self, id: i32, in_use: bool, conn: &mut DbConnection) -> Result<(), DbError> {
        update(dsl_datasets)
            .filter(dsl_id.eq(id))
            .set(dsl_in_use.eq(true))
            .execute(conn)
            .map(|_| ())
            .map_err(StdError::from)
            .map_err(|e| DbError::source(UpdateFailed, "Could not set in_use", e))
    }

    fn delete_datasets(&self, conn: &mut DbConnection) -> Result<(), DbError> {
        delete(dsl_datasets)
            .execute(conn)
            .map(|_| ())
            .map_err(StdError::from)
            .map_err(|e| DbError::source(DeleteFailed, "Could not delete datasets", e))
    }

    fn delete_dataset(&self, id: i32, conn: &mut DbConnection) -> Result<(), DbError> {
        delete(dsl_datasets.filter(dsl_id.eq(id)))
            .execute(conn)
            .map(|_| ())
            .map_err(StdError::from)
            .map_err(|e| DbError::source(DeleteFailed, "Could not delete dataset", e))
    }
}

impl DatasetRepository for PostgresDatasetRepository {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, DbError> {
        dsl_datasets
            .select((dsl_id, dsl_in_use, dsl_created_on))
            .load::<DatasetInfo>(&mut self.connection()?)
            .map_err(StdError::from)
            .map_err(|e| DbError::source(ReadFailed, "Datasets not found", e))
    }

    fn dataset_data(&self, id: i32) -> Result<Vec<u8>, DbError> {
        dsl_datasets
            .filter(dsl_id.eq(id))
            .select(dsl_data)
            .get_result::<Vec<u8>>(&mut self.connection()?)
            .map_err(StdError::from)
            .map_err(|e| DbError::source(ReadFailed, "Dataset not found", e))
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

    fn set_in_use_dataset(&self, id: i32) -> Result<(), DbError> {
        let mut conn = self.connection()?;
        conn.transaction::<_, DbError, _>(|conn| {
            let info = self.dataset(id, conn)?;
            if !info.in_use {
                self.update_all_not_in_use(conn)?;
                self.update_in_use(info.id, true, conn)?;
            }
            Ok(())
        })
    }

    fn delete_datasets(&self) -> Result<(), DbError> {
        let mut conn = self.connection()?;
        conn.transaction::<_, DbError, _>(|conn| self.delete_datasets(conn))
    }

    fn delete_dataset(&self, id: i32) -> Result<(), DbError> {
        let mut conn = self.connection()?;
        conn.transaction::<_, DbError, _>(|conn| self.delete_dataset(id, conn))
    }
}
