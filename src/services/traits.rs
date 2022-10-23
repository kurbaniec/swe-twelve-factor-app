use crate::entities::datasets::{DatasetInfo, DatasetUpload};
use crate::errors::service_error::ServiceError;
use rocket::form::Form;
use std::path::Path;

pub trait Classify {
    fn valid_dataset(&self, dataset: &Path) -> Result<(), ServiceError>;
    fn load_dataset(&self, dataset: &Path) -> Result<(), ServiceError>;
}

pub trait Manage {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, ServiceError>;
    fn add_dataset(&self, upload: Form<DatasetUpload<'_>>) -> Result<DatasetInfo, ServiceError>;
}
