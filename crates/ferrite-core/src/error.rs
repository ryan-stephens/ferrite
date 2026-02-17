use thiserror::Error;

#[derive(Debug, Error)]
pub enum FerriteError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Scanner error: {0}")]
    Scanner(String),

    #[error("Metadata error: {0}")]
    Metadata(String),

    #[error("Transcode error: {0}")]
    Transcode(String),

    #[error("Stream error: {0}")]
    Stream(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}

pub type FerriteResult<T> = Result<T, FerriteError>;
