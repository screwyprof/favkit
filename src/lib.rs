#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod favorites;
pub mod finder;
pub mod system;

pub use finder::FinderApi;
pub use system::{Favorites, RealMacOsApi};
