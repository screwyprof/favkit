use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur when interacting with the macOS Finder sidebar.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to convert a URL string to a valid path.
    #[error("Failed to convert URL '{url}' to a valid path")]
    UrlConversion { url: String },

    /// The provided path is not valid for the operation.
    #[error("Invalid path '{path}': must be an absolute path that exists")]
    InvalidPath { path: PathBuf },

    /// A Core Foundation operation failed.
    #[error("Core Foundation operation failed: {kind}")]
    CoreFoundation { kind: CoreFoundationErrorKind },

    /// Failed to get the favorites list from the system.
    #[error("Failed to get favorites list: {reason}")]
    GetFavoritesList { reason: &'static str },

    /// Failed to get an item's display name.
    #[error("Failed to get display name for item: {reason}")]
    GetDisplayName { reason: &'static str },

    /// Failed to get an item's URL.
    #[error("Failed to get URL for item: {reason}")]
    GetItemUrl { reason: &'static str },
}

/// Specific kinds of Core Foundation errors that can occur.
#[derive(Debug)]
pub enum CoreFoundationErrorKind {
    /// A null pointer was encountered where a valid pointer was expected.
    NullPointer,
    /// A reference to a Core Foundation object was invalid.
    InvalidReference,
    /// Failed to convert between Core Foundation types.
    ConversionFailed,
}

impl std::fmt::Display for CoreFoundationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullPointer => write!(f, "unexpected null pointer"),
            Self::InvalidReference => write!(f, "invalid Core Foundation reference"),
            Self::ConversionFailed => write!(f, "type conversion failed"),
        }
    }
}

/// A specialized Result type for operations that can fail with a sidebar-related error.
pub type Result<T> = std::result::Result<T, Error>;
