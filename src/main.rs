#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::{FinderApi, RealMacOsApi, finder::Result};

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<()> {
    let macos_api = RealMacOsApi::new();
    let api = FinderApi::new(&macos_api);

    let _favorites = api.get_favorites_list()?;

    Ok(())
}
