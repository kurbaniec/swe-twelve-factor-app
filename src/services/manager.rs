use crate::entities::datasets::{DatasetInfo, DatasetInsert, DatasetUpload};
use crate::errors::app_error::AppError;
use crate::errors::db_error::DbErrorKind;
use crate::errors::db_error::DbErrorKind::ReadFailed;
use crate::errors::service_error::ServiceError;
use crate::errors::std_error::StdError;
use crate::services::traits::Manage;
use crate::states::app_state::{DatasetRepoPtr, ImageClassifierPtr};
use crate::utils::zip::unzip;
use rocket::form::Form;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
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

    fn dataset_data(&self, id: i32) -> Result<PathBuf, ServiceError> {
        let data = self.db.dataset_data(id).map_err(|e| match e.kind {
            ReadFailed => ServiceError::illegal_argument_src("No such dataset", e),
            _ => ServiceError::crud_failed_src("Could not query dataset data", e),
        })?;

        persist_dataset(&data, id.to_string())
    }

    fn add_dataset(&self, upload: DatasetUpload<'_>) -> Result<DatasetInfo, ServiceError> {
        let archive_path = upload
            .data
            .path()
            .ok_or_else(|| ServiceError::dataset_failure("Could not find uploaded dataset"))?;
        let dataset = unzip_dataset(archive_path)?;

        self.ic.validate_dataset(&dataset)?;

        let _ = removed_unzipped_dataset(&dataset);
        let insert = DatasetInsert::from(upload);
        self.db
            .add_dataset(&insert)
            .map_err(|e| ServiceError::crud_failed_src("Could not store dataset", e))
    }

    fn set_in_use_dataset(&self, id: i32) -> Result<(), ServiceError> {
        self.db.set_in_use_dataset(id).map_err(|e| match e.kind {
            ReadFailed => ServiceError::illegal_argument_src("No such dataset", e),
            _ => ServiceError::crud_failed_src("Could not update dataset", e),
        })
    }

    fn load_dataset(&self, id: i32) -> Result<(), ServiceError> {
        let data = self.db.dataset_data(id).map_err(|e| match e.kind {
            ReadFailed => ServiceError::illegal_argument_src("No such dataset", e),
            _ => ServiceError::crud_failed_src("Could not query dataset data", e),
        })?;
        let archive_path = persist_dataset(&data, id.to_string())?;
        let dataset = unzip_dataset(&archive_path)?;
        let load_result = self.ic.load_dataset(&dataset);
        let _ = removed_unzipped_dataset(&dataset);
        load_result
    }

    fn load_latest_dataset(&self) -> Result<(), ServiceError> {
        let data = self.db.dataset_latest_data().map_err(|e| match e.kind {
            ReadFailed => ServiceError::illegal_argument_src("No such dataset", e),
            _ => ServiceError::crud_failed_src("Could not query dataset data", e),
        })?;
        let archive_path = persist_dataset(&data, String::from("latest"))?;
        let dataset = unzip_dataset(&archive_path)?;
        let load_result = self.ic.load_dataset(&dataset);
        let _ = removed_unzipped_dataset(&dataset);
        load_result
    }

    fn delete_datasets(&self) -> Result<(), ServiceError> {
        self.db
            .delete_datasets()
            .map_err(|e| ServiceError::crud_failed_src("Could not delete datasets", e))
    }

    fn delete_dataset(&self, id: i32) -> Result<(), ServiceError> {
        self.db
            .delete_dataset(id)
            .map_err(|e| ServiceError::crud_failed_src("Could not delete datasets", e))
    }
}

fn persist_dataset(data: &Vec<u8>, name: String) -> Result<PathBuf, ServiceError> {
    let base_path = env::temp_dir().join(Uuid::new_v4().to_string());
    let path = base_path.join(format!("dataset-{}.zip", name));
    fs::create_dir_all(base_path)
        .map_err(StdError::from)
        .map_err(|e| ServiceError::dataset_failure_src("Could not create dataset path", e))?;
    let mut file = fs::File::create(&path)
        .map_err(StdError::from)
        .map_err(|e| ServiceError::dataset_failure_src("Could not find dataset path", e))?;
    file.write_all(&data)
        .map_err(StdError::from)
        .map_err(|e| ServiceError::dataset_failure_src("Could not store dataset", e))?;
    Ok(path)
}

fn unzip_dataset(path: &Path) -> Result<PathBuf, ServiceError> {
    let archive_file = fs::File::open(path)
        .map_err(StdError::from)
        .map_err(|e| ServiceError::dataset_failure_src("Could not find datasets file", e))?;

    let mut archive = zip::ZipArchive::new(archive_file)
        .map_err(StdError::from)
        .map_err(|e| ServiceError::dataset_failure_src("Could not find dataset archive", e))?;

    let target_path = path
        .parent()
        .map(|it| it.join(Uuid::new_v4().to_string()))
        .ok_or_else(|| ServiceError::dataset_failure("Could not create dataset directory"))?;

    fs::create_dir_all(&target_path)
        .map_err(StdError::from)
        .map_err(|e| ServiceError::dataset_failure_src("Could not find dataset archive", e))?;

    unzip(&mut archive, &target_path)
        .map(|_| target_path)
        .map_err(|e| ServiceError::dataset_failure_src("Could not unzip dataset", e))
}

fn removed_unzipped_dataset(path: &Path) -> Result<(), ServiceError> {
    fs::remove_dir_all(&path)
        .map_err(StdError::from)
        .map_err(|e| {
            println!("   >> Remove directory {:?} manually", path);
            e.print_stacktrace();
            ServiceError::dataset_failure_src("Could not remove dataset", e)
        })
}
