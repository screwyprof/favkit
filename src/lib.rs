#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod errors;
pub mod finder;

// Re-export key types
pub use finder::macos::MacOsApi;
pub use finder::repository::Repository;
pub use finder::target::Target;
pub use finder::Finder;
