#[macro_use]
extern crate rocket;
extern crate core;

use crate::repositories::dataset_repository::DatasetRepository;
use crate::services::image_classifier::ImageClassifier;
use crate::services::manager::Manager;
use std::borrow::Borrow;
use std::env::current_exe;
use tensorflow::{Graph, SavedModelBundle, SessionOptions};

mod entities;
mod errors;
mod repositories;
mod routes;
mod services;

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let image_classifier = ImageClassifier::new();
//     let dataset_db = DatasetRepository::new();
//     let manager = Manager::new(image_classifier.clone(), dataset_db.clone());
//
//     let _ = rocket::build()
//         .manage(image_classifier)
//         .manage(manager)
//         .mount(
//             "/",
//             routes![routes::service::index, routes::service::is_dog],
//         )
//         .launch()
//         .await?;
//     Ok(())
// }

fn main() {
    // Based on: https://www.christianhaller.me/blog/projectblog/2020-06-02-TFCatsVsDogsI/
    // Big Thanks:
    // https://towardsdatascience.com/training-keras-models-using-the-rust-tensorflow-bindings-941791249a7
    // https://stackoverflow.com/questions/68199756/loading-a-trained-hdf5-model-into-rust-to-make-predictions
    // https://cetra3.github.io/blog/face-detection-with-tensorflow-rust/
    let input_parameter_name = "image_input";
    let output_parameter_name = "output";

    let save_dir = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("dogorcat")
        .into_os_string()
        .into_string()
        .unwrap();

    let mut graph = Graph::new();
    let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, save_dir);
}
