#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::{finder::repository::Repository, SystemMacOsApi};

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = Box::new(SystemMacOsApi::new());
    let repository = Repository::new(api);
    match repository.load() {
        Ok(sidebar) => {
            println!("Favorites:");
            for favorite in sidebar.favorites() {
                println!("- {} ({})", favorite.display_name(), favorite.target());
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("Error loading favorites: {}", err);
            Err(Box::new(err))
        }
    }
}
