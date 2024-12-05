use thiserror::Error;

/// All possible errors that can occur in the application
#[derive(Debug, Error)]
pub enum Error {
}

/// A specialized Result type for our operations
pub type Result<T> = std::result::Result<T, Error>;
