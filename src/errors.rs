use std::path::PathBuf;
use thiserror::Error;
use crate::finder::macos_url::UrlError;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("Invalid path: {path}")]
    InvalidPath {
        path: PathBuf,
        #[source]
        source: Option<std::io::Error>,
    },
    #[error("Unsupported target: {0}")]
    UnsupportedTarget(String),
    #[error("System error: {0}")]
    SystemError(String),
    #[error(transparent)]
    Url(#[from] UrlError),
}

impl FinderError {
    pub fn invalid_path(path: impl Into<PathBuf>) -> Self {
        Self::InvalidPath {
            path: path.into(),
            source: None,
        }
    }

    pub fn with_source(mut self, source: std::io::Error) -> Self {
        if let FinderError::InvalidPath { source: ref mut err, .. } = self {
            *err = Some(source);
        }
        self
    }
}

pub type Result<T> = std::result::Result<T, FinderError>;
