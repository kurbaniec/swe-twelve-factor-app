use crate::entities::datasets::{DatasetInfo, DatasetInsert};
use crate::errors::db_error::DbError;

pub trait DatasetRepository {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, DbError>;
    fn add_dataset(&self, dataset: &DatasetInsert) -> Result<DatasetInfo, DbError>;
}
