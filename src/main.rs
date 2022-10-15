#[macro_use]
extern crate rocket;

use crate::services::image_classifier::ImageClassifier;
use crate::services::manager::Manager;

mod entities;
mod errors;
mod routes;
mod services;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let image_classifier = ImageClassifier::new();
    let manager = Manager::new(image_classifier.clone());

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
