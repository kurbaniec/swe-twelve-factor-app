use crate::services::image_classifier::{ImageClassifier, ImageClassifierArc};
use std::sync::Arc;

pub struct Manager {
    image_classifier: ImageClassifierArc,
}

pub type ManagerArc = Arc<Manager>;

impl Manager {
    pub fn new(image_classifier: ImageClassifierArc) -> ManagerArc {
        Arc::from(Manager { image_classifier })
    }

    pub fn test(&self) {
        println!("Hey!")
    }
}
