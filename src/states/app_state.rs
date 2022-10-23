use crate::repositories::traits::DatasetRepository;
use crate::services::traits::{Classify, Manage};
use crate::{ImageClassifier, Manager, PostgresDatasetRepository};
use rocket::State;
use std::error::Error;
use std::sync::Arc;

pub type ImageClassifierPtr = Arc<dyn Classify + Send + Sync>;
pub type DatasetRepoPtr = Arc<dyn DatasetRepository + Send + Sync>;
pub type ManagerPtr = Arc<dyn Manage + Send + Sync>;

pub type ImageClassifierState = State<ImageClassifierPtr>;
pub type ManagerState = State<ManagerPtr>;

pub struct AppState {
    ic: ImageClassifierPtr,
    db: DatasetRepoPtr,
    m: ManagerPtr,
}

impl AppState {
    pub fn new() -> Self {
        let ic: ImageClassifierPtr = Arc::from(ImageClassifier::new());
        let db: DatasetRepoPtr = Arc::from(PostgresDatasetRepository::new());
        let m: ManagerPtr = Arc::from(Manager::new(ic.clone(), db.clone()));
        AppState { ic, db, m }
    }

    pub fn get_image_classifier(&self) -> ImageClassifierPtr {
        self.ic.clone()
    }

    pub fn get_dataset_repository(&self) -> DatasetRepoPtr {
        self.db.clone()
    }

    pub fn get_manager(&self) -> ManagerPtr {
        self.m.clone()
    }
}