use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to convert URL: {url}")]
    UrlConversion { url: String },

    #[error("Invalid path: {path}")]
    InvalidPath { path: PathBuf },

    #[error("Core Foundation error: {kind}")]
    CoreFoundation { kind: CoreFoundationErrorKind },

    #[error("Failed to get favorites list")]
    GetFavoritesList,

    #[error("Failed to get item display name")]
    GetDisplayName,

    #[error("Failed to get item URL")]
    GetItemUrl,
}

#[derive(Debug)]
pub enum CoreFoundationErrorKind {
    NullPointer,
    InvalidReference,
    ConversionFailed,
}

impl std::fmt::Display for CoreFoundationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullPointer => write!(f, "null pointer"),
            Self::InvalidReference => write!(f, "invalid reference"),
            Self::ConversionFailed => write!(f, "type conversion failed"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
