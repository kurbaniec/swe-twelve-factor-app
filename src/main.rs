#[macro_use]
extern crate rocket;
extern crate core;

use crate::repositories::dataset_repository::DatasetRepository;
use crate::services::image_classifier::ImageClassifier;
use crate::services::manager::Manager;

mod entities;
mod errors;
mod repositories;
mod routes;
mod services;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let image_classifier = ImageClassifier::new();
    let dataset_db = DatasetRepository::new();
    let manager = Manager::new(image_classifier.clone(), dataset_db.clone());

    let _ = rocket::build()
        .manage(image_classifier)
        .manage(manager)
        .mount(
            "/",
            routes![routes::service::index, routes::service::is_dog],
        )
        .launch()
        .await?;
    Ok(())
}
