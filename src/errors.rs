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

    #[error("Failed to get favorites snapshot: {kind}")]
    FavoritesError {
        kind: FavoritesErrorKind,
    },
}

#[derive(Debug)]
pub enum FavoritesErrorKind {
    FailedToGetList,
    FailedToGetSnapshot,
}

impl std::fmt::Display for FavoritesErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToGetList => write!(f, "could not get favorites list"),
            Self::FailedToGetSnapshot => write!(f, "could not get favorites snapshot"),
        }
    }
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
