#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::errors::Result;
use favkit::{Finder, SystemMacOsApi, Repository};

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<()> {
    let api = Box::new(SystemMacOsApi::new());
    let repository = Repository::new(api);
    let sidebar = repository.load()?;
    let finder = Finder::new(sidebar);

    println!("Favorites:");
    for favorite in finder.sidebar() {
        println!("- {} ({})", favorite.display_name(), favorite.target());
    }
    Ok(())
}
