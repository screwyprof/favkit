#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::finder::FinderApi;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    let finder = FinderApi::default();

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
