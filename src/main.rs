#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::{Favorites, FinderApi, RealMacOsApi, finder::Result};

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<()> {
    let macos_api = RealMacOsApi::new();
    let favorites = Favorites::new(&macos_api);
    let finder = FinderApi::new(&favorites);

    let _favorites = finder.get_favorites_list()?;

    Ok(())
}
