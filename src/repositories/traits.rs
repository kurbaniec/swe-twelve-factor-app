use crate::entities::datasets::DatasetInfo;
use crate::errors::db_error::DbError;


pub trait DatasetRepository {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, DbError>;
}
