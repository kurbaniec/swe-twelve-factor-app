use crate::entities::image::{ImageClassification, ImageUpload};
use crate::errors::app_error::AppError;
use crate::errors::service_error::ServiceError;
use crate::errors::std_error::StdError;
use crate::services::traits::Classify;
use image::imageops::FilterType;
use image::GenericImageView;
use rocket::fs::TempFile;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use tensorflow::{Graph, SavedModelBundle, Session, SessionOptions, SessionRunArgs, Tensor};
use uuid::Uuid;

const IMAGE_WIDTH: u64 = 150;
const IMAGE_HEIGHT: u64 = 150;
const IMAGE_COLOR_SPACE: u64 = 3; //RGB

// Based on: https://www.christianhaller.me/blog/projectblog/2020-06-02-TFCatsVsDogsI/
// Big Thanks:
// https://towardsdatascience.com/training-keras-models-using-the-rust-tensorflow-bindings-941791249a7
// https://stackoverflow.com/questions/68199756/loading-a-trained-hdf5-model-into-rust-to-make-predictions
// https://cetra3.github.io/blog/face-detection-with-tensorflow-rust/
pub struct ImageClassifier {
    tf: RwLock<Option<TensorflowClassifier>>,
}

impl ImageClassifier {
    pub fn new() -> Self {
        ImageClassifier {
            tf: RwLock::new(None),
        }
    }
}

impl Classify for ImageClassifier {
    fn load_dataset(&self, dataset: &Path) -> Result<(), ServiceError> {
        let dataset = dataset_directory(dataset)?;
        let tf = TensorflowClassifier::build(&dataset)?;
        let mut guard = self.tf.write().unwrap();
        *guard = Some(tf);
        Ok(())
    }

    fn classify_image(&self, image: ImageUpload) -> Result<ImageClassification, ServiceError> {
        let guard = self.tf.read().unwrap();
        if let Some(ref tf) = *guard {
            let image_path = persist_image(&image.image)?;
            let input = image_to_tensor(&image_path)?;
            let _ = remove_image(&image_path);
            tf.classify_image(&input)
        } else {
            Err(ServiceError::no_dataset("Dataset not loaded"))
        }
    }

    fn validate_dataset(&self, dataset: &Path) -> Result<(), ServiceError> {
        let dataset = dataset_directory(dataset)?;
        let tf = TensorflowClassifier::build(&dataset)?;
        let image = test_tensor();
        tf.classify_image(&image).map(|_| ())
    }
}

struct TensorflowClassifier {
    pub graph: Graph,
    pub bundle: SavedModelBundle,
}

impl TensorflowClassifier {
    fn build(path: &Path) -> Result<Self, ServiceError> {
        let mut graph = Graph::new();
        let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, path)
            .map_err(StdError::from)
            .map_err(|e| ServiceError::illegal_argument_src("Unsupported dataset", e))?;

        Ok(TensorflowClassifier { graph, bundle })
    }

    fn classify_image(&self, input: &Tensor<f32>) -> Result<ImageClassification, ServiceError> {
        let signature_input_parameter_name = "image_input";
        let signature_output_parameter_name = "output";

        let session = &self.bundle.session;

        let signature = self
            .bundle
            .meta_graph_def()
            .get_signature("serving_default")
            .unwrap();

        let input_info = signature.get_input(signature_input_parameter_name).unwrap();
        let output_info = signature
            .get_output(signature_output_parameter_name)
            .unwrap();

        let input_op = self
            .graph
            .operation_by_name_required(&input_info.name().name)
            .unwrap();
        let output_op = self
            .graph
            .operation_by_name_required(&output_info.name().name)
            .unwrap();

        let mut args = SessionRunArgs::new();
        args.add_feed(&input_op, 0, input);
        let out = args.request_fetch(&output_op, 0);

        session
            .run(&mut args)
            .map_err(StdError::from)
            .map_err(|e| ServiceError::illegal_argument_src("Unsupported input", e))?;

        let output: f32 = args
            .fetch(out)
            .map(|it| it[0])
            .map_err(StdError::from)
            .map_err(|e| ServiceError::illegal_argument_src("Unsupported output", e))?;

        if output > 0.0 {
            Ok(ImageClassification::dog())
        } else {
            Ok(ImageClassification::cat())
        }
    }
}

fn persist_image(tmp: &TempFile) -> Result<PathBuf, ServiceError> {
    let content_type = tmp.content_type().ok_or_else(|| {
        ServiceError::image_prep_failed("Not a suitable image for classification")
    })?;
    let extension = content_type.extension().ok_or_else(|| {
        ServiceError::image_prep_failed("Not a suitable image for classification")
    })?;
    let path = tmp
        .path()
        .ok_or_else(|| ServiceError::image_prep_failed("Could not determine image"))?;
    let parent_path = path
        .parent()
        .ok_or_else(|| ServiceError::image_prep_failed("Could not determine image"))?;
    let image_path = parent_path.join(format!("{}.{}", Uuid::new_v4(), extension));
    fs::copy(&path, &image_path)
        .map_err(StdError::from)
        .map_err(|e| ServiceError::image_prep_failed_src("Could not copy image", e))?;
    Ok(image_path)
}

fn remove_image(path: &Path) -> Result<(), ServiceError> {
    fs::remove_file(&path).map_err(StdError::from).map_err(|e| {
        println!("   >> Remove file {:?} manually", path);
        e.print_stacktrace();
        ServiceError::dataset_failure_src("Could not remove dataset", e)
    })
}

fn image_to_tensor(path: &Path) -> Result<Tensor<f32>, ServiceError> {
    let image = image::open(path).map_err(StdError::from).map_err(|e| {
        ServiceError::illegal_argument_src("Not a suitable image for classification", e)
    })?;
    let mut image_pixels: Vec<f32> = Vec::new();
    let image = image.resize_exact(
        IMAGE_WIDTH as u32,
        IMAGE_HEIGHT as u32,
        FilterType::Lanczos3,
    );
    for (_x, _y, rgb) in image.pixels() {
        image_pixels.push(rgb[0] as f32);
        image_pixels.push(rgb[1] as f32);
        image_pixels.push(rgb[2] as f32);
    }

    Tensor::new(&[
        1, // Only one picture
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        IMAGE_COLOR_SPACE,
    ])
    .with_values(&image_pixels)
    .map_err(StdError::from)
    .map_err(|e| ServiceError::illegal_argument_src("Could not create tensor", e))
}

fn test_tensor() -> Tensor<f32> {
    let size: usize = (IMAGE_WIDTH * IMAGE_HEIGHT * IMAGE_COLOR_SPACE) as usize;
    let fake_image: Vec<f32> = vec![0.0; size];
    Tensor::new(&[
        1, // Only one picture
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        IMAGE_COLOR_SPACE,
    ])
    .with_values(&fake_image)
    .unwrap()
}

// Search for directory that contains ".pb" or ".pbtxt" (tensorflow starting point)
fn dataset_directory(path: &Path) -> Result<PathBuf, ServiceError> {
    let result = dataset_directory_helper(path)?;
    if let Some(path) = result {
        Ok(path)
    } else {
        Err(ServiceError::illegal_argument(
            "Could not find valid dataset",
        ))
    }
}

fn dataset_directory_helper(path: &Path) -> Result<Option<PathBuf>, ServiceError> {
    if path.is_dir() {
        let entries = fs::read_dir(path)
            .map_err(StdError::from)
            .map_err(|e| ServiceError::illegal_argument_src("Could not find valid dataset", e))?;
        for entry in entries {
            let entry = entry.map_err(StdError::from).map_err(|e| {
                ServiceError::illegal_argument_src("Could not find valid dataset", e)
            })?;
            let path = entry.path();
            if path.is_dir() {
                let result = dataset_directory_helper(&path)?;
                if result.is_some() {
                    return Ok(result);
                }
            } else if let Some(ext) = path.extension() {
                match ext.to_str() {
                    Some("pb") => return Ok(Some(PathBuf::from(path.parent().unwrap()))),
                    Some("pbtxt") => return Ok(Some(PathBuf::from(path.parent().unwrap()))),
                    _ => {}
                }
            }
        }
    }
    Err(ServiceError::illegal_argument(
        "Could not find valid dataset",
    ))
}
