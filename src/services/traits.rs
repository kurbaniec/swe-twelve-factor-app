use crate::entities::datasets::{DatasetInfo, DatasetUpload};
use crate::errors::service_error::ServiceError;

use std::path::Path;

pub trait Classify {
    fn valid_dataset(&self, dataset: &Path) -> Result<(), ServiceError>;
    fn load_dataset(&self, dataset: &Path) -> Result<(), ServiceError>;
}

pub trait Manage {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, ServiceError>;
    fn dataset_data(&self, id: i32) -> Result<&Path, ServiceError>;
    fn add_dataset(&self, upload: DatasetUpload<'_>) -> Result<DatasetInfo, ServiceError>;
    fn set_in_use_dataset(&self, id: i32) -> Result<(), ServiceError>;
    fn delete_datasets(&self) -> Result<(), ServiceError>;
    fn delete_dataset(&self, id: i32) -> Result<(), ServiceError>;
}
