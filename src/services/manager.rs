use crate::entities::datasets::DatasetInfo;
use crate::errors::service_error::ServiceError;
use crate::services::traits::{Classify, Manage};
use crate::states::app_state::{DatasetRepoPtr, ImageClassifierPtr};
use std::sync::Arc;

pub struct Manager {
    ic: ImageClassifierPtr,
    db: DatasetRepoPtr,
}

impl Manager {
    pub fn new(image_classifier: ImageClassifierPtr, dataset_db: DatasetRepoPtr) -> Self {
        Manager {
            ic: image_classifier,
            db: dataset_db,
        }
    }
}

impl Manage for Manager {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, ServiceError> {
        self.db
            .datasets()
            .map_err(|e| ServiceError::crud_failed("Could not query datasets", e))
    }
}
