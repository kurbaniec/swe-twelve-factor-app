use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{r2d2, Connection, PgConnection};
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
}
