use crate::services::traits::Classify;
use std::sync::Arc;

pub struct ImageClassifier {}

impl ImageClassifier {
    pub fn new() -> Self {
        ImageClassifier {}
    }

    pub fn test(&self) {
        println!("Hey!")
    }
}

impl Classify for ImageClassifier {}
unsafe impl Send for ImageClassifier {}
unsafe impl Sync for ImageClassifier {}
