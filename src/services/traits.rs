use crate::entities::datasets::{DatasetInfo, DatasetUpload};
use crate::errors::service_error::ServiceError;

use crate::entities::image::{ImageClassification, ImageUpload};
use std::path::{Path, PathBuf};

pub trait Classify {
    fn load_dataset(&self, dataset: &Path) -> Result<(), ServiceError>;
    fn classify_image(&self, image: ImageUpload) -> Result<ImageClassification, ServiceError>;
    fn validate_dataset(&self, dataset: &Path) -> Result<(), ServiceError>;
}

pub trait Manage {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, ServiceError>;
    fn dataset_data(&self, id: i32) -> Result<PathBuf, ServiceError>;
    fn add_dataset(&self, upload: DatasetUpload<'_>) -> Result<DatasetInfo, ServiceError>;
    fn set_in_use_dataset(&self, id: i32) -> Result<(), ServiceError>;
    fn load_dataset(&self, id: i32) -> Result<(), ServiceError>;
    fn load_latest_dataset(&self) -> Result<(), ServiceError>;
    fn delete_datasets(&self) -> Result<(), ServiceError>;
    fn delete_dataset(&self, id: i32) -> Result<(), ServiceError>;
}
