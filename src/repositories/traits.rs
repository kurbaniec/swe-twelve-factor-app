use crate::entities::datasets::{DatasetInfo, DatasetInsert};
use crate::errors::db_error::DbError;

pub trait DatasetRepository {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, DbError>;
    fn dataset_data(&self, id: i32) -> Result<Vec<u8>, DbError>;
    fn dataset_latest_data(&self) -> Result<Vec<u8>, DbError>;
    fn add_dataset(&self, dataset: &DatasetInsert) -> Result<DatasetInfo, DbError>;
    fn set_in_use_dataset(&self, id: i32) -> Result<(), DbError>;
    fn delete_datasets(&self) -> Result<(), DbError>;
    fn delete_dataset(&self, id: i32) -> Result<(), DbError>;
}
