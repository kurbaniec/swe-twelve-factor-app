use crate::entities::datasets::{DatasetInfo, DatasetInsert, DatasetUpload};
use crate::errors::app_error::AppError;
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
        let archive_path = upload
            .data
            .path()
            .ok_or_else(|| ServiceError::dataset_failure("Could not find uploaded dataset"))?;

        let archive_file = fs::File::open(archive_path)
            .map_err(StdError::from)
            .map_err(|e| ServiceError::dataset_failure_src("Could not find datasets file", e))?;

        let mut archive = zip::ZipArchive::new(archive_file)
            .map_err(StdError::from)
            .map_err(|e| ServiceError::dataset_failure_src("Could not find dataset archive", e))?;

        let target_base_path = archive_path
            .parent()
            .map(|it| it.join(Uuid::new_v4().to_string()))
            .ok_or_else(|| ServiceError::dataset_failure("Could not create dataset directory"))?;
        let target_path = target_base_path.join("dataset");

        fs::create_dir_all(&target_path)
            .map_err(StdError::from)
            .map_err(|e| ServiceError::dataset_failure_src("Could not find dataset archive", e))?;

        unzip(&mut archive, &target_path)
            .map_err(|e| ServiceError::dataset_failure_src("Could not unzip dataset", e))?;

        println!("{:?}", &target_path);

        self.ic.valid_dataset(&target_path)?;

        fs::remove_dir_all(target_base_path)
            .map_err(StdError::from)
            .unwrap_or_else(|e| {
                e.print_stacktrace();
            });

        let insert = DatasetInsert::from(upload.into_inner());
        self.db
            .add_dataset(&insert)
            .map_err(|e| ServiceError::dataset_failure_src("Could not store dataset", e))
    }
}
