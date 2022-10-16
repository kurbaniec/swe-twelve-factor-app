use crate::entities::datasets::DatasetInfo;
use crate::errors::serviceerror::ServiceError;
use crate::repositories::dataset_repository::DatasetRepositoryArc;
use crate::services::image_classifier::{ImageClassifier, ImageClassifierArc};
use std::sync::Arc;

pub struct Manager {
    ic: ImageClassifierArc,
    db: DatasetRepositoryArc,
}

pub type ManagerArc = Arc<Manager>;

impl Manager {
    pub fn new(
        image_classifier: ImageClassifierArc,
        dataset_db: DatasetRepositoryArc,
    ) -> ManagerArc {
        Arc::from(Manager {
            ic: image_classifier,
            db: dataset_db,
        })
    }

    pub fn datasets(&self) -> Result<Vec<DatasetInfo>, ServiceError> {
        self.db
            .datasets()
            .map_err(|e| ServiceError::crud_failed("Could not query datasets", e))
    }
}
