use std::sync::Arc;

pub struct ImageClassifier {}

pub type ImageClassifierArc = Arc<ImageClassifier>;

impl ImageClassifier {
    pub fn new() -> ImageClassifierArc {
        Arc::from(ImageClassifier {})
    }

    pub fn test(&self) {
        println!("Hey!")
    }
}
