#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod finder;

pub use finder::{FinderApi, RealMacOsApi};
