#[macro_use]
extern crate rocket;
extern crate core;

use crate::repositories::dataset_repository::PostgresDatasetRepository;
use crate::services::image_classifier::ImageClassifier;
use crate::services::manager::Manager;
use crate::states::app_state::AppState;

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
                routes::service::index,
                routes::management::datasets,
                routes::management::add_dataset
            ],
        )
        .launch()
        .await?;
    Ok(())
}

// fn main() {
//     // Based on: https://www.christianhaller.me/blog/projectblog/2020-06-02-TFCatsVsDogsI/
//     // Big Thanks:
//     // https://towardsdatascience.com/training-keras-models-using-the-rust-tensorflow-bindings-941791249a7
//     // https://stackoverflow.com/questions/68199756/loading-a-trained-hdf5-model-into-rust-to-make-predictions
//     // https://cetra3.github.io/blog/face-detection-with-tensorflow-rust/
//     let signature_input_parameter_name = "image_input";
//     let signature_output_parameter_name = "output";
//
//     let save_dir = current_exe()
//         .unwrap()
//         .parent()
//         .unwrap()
//         .join("dogorcat")
//         .into_os_string()
//         .into_string()
//         .unwrap();
//
//     let mut graph = Graph::new();
//     let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, save_dir)
//         .expect("Can't load saved model");
//
//     let session = &bundle.session;
//     let signature = bundle
//         .meta_graph_def()
//         .get_signature("serving_default")
//         .unwrap();
//
//     let input_info = signature.get_input(signature_input_parameter_name).unwrap();
//     let output_info = signature
//         .get_output(signature_output_parameter_name)
//         .unwrap();
//
//     let input_op = graph
//         .operation_by_name_required(&input_info.name().name)
//         .unwrap();
//     let output_op = graph
//         .operation_by_name_required(&output_info.name().name)
//         .unwrap();
//
//     println!("{:?}", input_op.output_type(0));
//     println!("{:?}", output_op.output_type(0));
//
//     let image_path = current_exe().unwrap().parent().unwrap().join("5.jpg");
//     let image_input = image::open(&image_path.as_path()).unwrap();
//     let image_input = image_input.resize_exact(150, 150, FilterType::Lanczos3);
//     let mut image_pixels: Vec<f32> = Vec::new();
//     for (_x, _y, rgb) in image_input.pixels() {
//         image_pixels.push(rgb[0] as f32);
//         image_pixels.push(rgb[1] as f32);
//         image_pixels.push(rgb[2] as f32);
//     }
//
//     let input = Tensor::new(&[
//         1, // Only one picture
//         image_input.height() as u64,
//         image_input.width() as u64,
//         3,
//     ])
//     .with_values(&image_pixels)
//     .unwrap();
//
//     let mut args = SessionRunArgs::new();
//     args.add_feed(&input_op, 0, &input);
//     let out = args.request_fetch(&output_op, 0);
//
//     session.run(&mut args).unwrap();
//
//     let output: f32 = args.fetch(out).unwrap()[0];
//     if output > 0.0 {
//         println!("Is a Dog!")
//     } else {
//         println!("Is a Cat!")
//     }
// }
