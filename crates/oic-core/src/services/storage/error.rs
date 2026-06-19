use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    OpenDal(#[from] opendal::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
