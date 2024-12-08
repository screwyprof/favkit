use crate::finder::system::url::UrlError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("System error: {0}")]
    SystemError(String),

    #[error("Invalid path: {path}")]
    InvalidPath {
        path: PathBuf,
        source: Option<std::io::Error>,
    },

    #[error("Unsupported target: {0}")]
    UnsupportedTarget(String),

    #[error("URL error: {0}")]
    UrlError(#[from] UrlError),
}

impl FinderError {
    pub fn invalid_path(path: impl Into<PathBuf>) -> Self {
        Self::InvalidPath {
            path: path.into(),
            source: None,
        }
    }

    pub fn with_source(mut self, source: std::io::Error) -> Self {
        if let FinderError::InvalidPath {
            source: ref mut err,
            ..
        } = self
        {
            *err = Some(source);
        }
        self
    }
}

pub type Result<T> = std::result::Result<T, FinderError>;
