#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::{FinderApi, MacOsFavorites, RealMacOsApi, finder::Result};

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<()> {
    let macos_api = RealMacOsApi::new();
    let favorites = MacOsFavorites::new(&macos_api);
    let api = FinderApi::new(&favorites);

    let _favorites = api.get_favorites_list()?;

    Ok(())
}
