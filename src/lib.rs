#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub mod errors;
pub mod finder;

// Re-export key types
pub use finder::{
    Finder, MacOsApi, RealMacOsApi as SystemMacOsApi, Repository,
    sidebar::{Target, item::SidebarItem},
};
