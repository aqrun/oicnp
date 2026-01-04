use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    #[error("Cache entry expired: {0}")]
    Expired(String),
    
    #[error("Compression error: {0}")]
    Compression(String),
    
    #[error("Concurrent write conflict: {0}")]
    WriteConflict(String),
    
    #[error("Fetch timeout: {0}")]
    FetchTimeout(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

pub type Result<T> = std::result::Result<T, CacheError>;

