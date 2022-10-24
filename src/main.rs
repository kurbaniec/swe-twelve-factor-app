#[macro_use]
extern crate rocket;
extern crate core;

use crate::repositories::dataset_repository::PostgresDatasetRepository;
use crate::services::image_classifier::ImageClassifier;
use crate::services::manager::Manager;
use crate::states::app_state::{AppState, ManagerPtr};
use crate::errors::app_error::AppError;
use crate::states::fairings::load_latest_dataset;

mod entities;
mod errors;
mod repositories;
mod routes;
mod services;
mod states;
mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let app_state = AppState::new();
    let image_classifier = app_state.get_image_classifier();
    let manager = app_state.get_manager();

    let _ = rocket::build()
        .manage(image_classifier)
        .manage(manager)
        .mount(
            "/",
            routes![
                routes::service::dog_or_cat,
                routes::management::datasets,
                routes::management::dataset_data,
                routes::management::add_dataset,
                routes::management::set_in_use_dataset,
                routes::management::load_dataset,
                routes::management::load_latest_dataset,
                routes::management::delete_datasets,
                routes::management::delete_dataset
            ],
        )
        .attach(load_latest_dataset())
        .launch()
        .await?;
    Ok(())
}
