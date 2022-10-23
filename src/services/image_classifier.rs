use crate::errors::service_error::ServiceError;
use crate::services::traits::Classify;
use std::path::Path;

pub struct ImageClassifier {}

impl ImageClassifier {
    pub fn new() -> Self {
        ImageClassifier {}
    }
}

impl Classify for ImageClassifier {
    fn valid_dataset(&self, dataset: &Path) -> Result<(), ServiceError> {
        println!("TODO Implement");
        Ok(())
    }

    fn load_dataset(&self, dataset: &Path) -> Result<(), ServiceError> {
        println!("TODO Implement");
        Ok(())
    }
}
