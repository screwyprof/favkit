#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::Finder;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    let finder = Finder::default();

    match finder.get_favorites_list() {
        Ok(items) => {
            for item in items {
                println!("{}", item);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
