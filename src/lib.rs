#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod errors;

use errors::Result;

/// FavKit is a library for managing macOS Finder sidebar items.
pub struct FavKit {
    // TODO: Add fields as needed
}

impl FavKit {
    /// Create a new FavKit instance
    pub fn new() -> Self {
        Self {}
    }

    /// Initialize FavKit with default settings
    pub fn init() -> Result<Self> {
        Ok(Self::new())
    }
}
