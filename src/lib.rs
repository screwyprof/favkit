#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod finder;
pub mod system;

pub use finder::Finder;
pub use system::RealMacOsApi;
