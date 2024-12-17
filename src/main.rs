#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::{
    finder::FinderApi,
    system::{Favorites, RealMacOsApi},
};

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    let api = RealMacOsApi::new();
    let favorites = Favorites::new(&api);
    let finder = FinderApi::new(&favorites);

    match finder.get_favorites_list() {
        Ok(items) => {
            println!("Found {} favorites:", items.len());
            for item in items {
                println!("  {}", item);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
