use crate::entities::datasets::{DatasetInfo, DatasetUpload};
use crate::errors::service_error::ServiceError;
use crate::errors::std_error::StdError;
use crate::services::traits::Manage;
use crate::states::app_state::{DatasetRepoPtr, ImageClassifierPtr};
use crate::utils::zip::unzip;
use rocket::form::Form;
use std::fs;
use uuid::Uuid;

pub struct Manager {
    ic: ImageClassifierPtr,
    db: DatasetRepoPtr,
}

impl Manager {
    pub fn new(image_classifier: ImageClassifierPtr, dataset_db: DatasetRepoPtr) -> Self {
        Manager {
            ic: image_classifier,
            db: dataset_db,
        }
    }
}

impl Manage for Manager {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, ServiceError> {
        self.db
            .datasets()
            .map_err(|e| ServiceError::crud_failed_src("Could not query datasets", e))
    }

    fn add_dataset(&self, upload: Form<DatasetUpload<'_>>) -> Result<DatasetInfo, ServiceError> {
        let path = upload
            .data
            .path()
            .ok_or_else(|| ServiceError::dataset_failure("Could not find uploaded dataset"))?;

        let file = fs::File::open(path)
            .map_err(|e| StdError::from(e))
            .map_err(|e| ServiceError::dataset_failure_src("Could not find datasets file", e))?;

        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| StdError::from(e))
            .map_err(|e| ServiceError::dataset_failure_src("Could not find dataset archive", e))?;

        let target_path = path
            .parent()
            .map(|it| it.join(Uuid::new_v4().to_string()))
            .map(|it| it.join("dataset"))
            .ok_or_else(|| ServiceError::dataset_failure("Could not create dataset directory"))?;
        fs::create_dir_all(&target_path)
            .map_err(|e| StdError::from(e))
            .map_err(|e| ServiceError::dataset_failure_src("Could not find dataset archive", e))?;

        println!("Target path: {:?}", &target_path);
        unzip(&mut archive, &target_path)
            .map_err(|e| ServiceError::dataset_failure_src("Could not unzip dataset", e))?;

        // TODO clean target path
        todo!()
    }
}
