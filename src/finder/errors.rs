use thiserror::Error;

use crate::system::favorites::FavoritesError;

#[derive(Debug, Error, PartialEq)]
pub enum FinderError {
    #[error("failed to access Finder favorites: {0}")]
    AccessError(#[from] FavoritesError),
}

pub type Result<T> = std::result::Result<T, FinderError>;
