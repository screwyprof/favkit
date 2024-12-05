use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, FinderError>;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("Invalid path: {path}")]
    InvalidPath {
        path: PathBuf,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Unsupported target path: {0}")]
    UnsupportedTarget(PathBuf),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl FinderError {
    pub fn invalid_path(path: impl Into<PathBuf>) -> Self {
        Self::InvalidPath {
            path: path.into(),
            source: None,
        }
    }
}
