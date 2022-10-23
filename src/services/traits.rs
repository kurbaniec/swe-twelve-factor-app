use crate::entities::datasets::DatasetInfo;
use crate::errors::service_error::ServiceError;

pub trait Classify {}

pub trait Manage {
    fn datasets(&self) -> Result<Vec<DatasetInfo>, ServiceError>;
}
