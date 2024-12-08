use crate::finder::system::url::UrlError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FinderError {
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

pub type Result<T> = std::result::Result<T, FinderError>;
