#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::{
    Finder,
    Repository,
};

mod finder;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    let api = Box::new(favkit::finder::macos_impl::SystemMacOsApi::new());
    let repository = Repository::new(api);
    let sidebar = repository.load();
    let finder = Finder::new(sidebar);

    println!("Finder Sidebar Items:");
    println!("--------------------");
    for item in finder.sidebar().favorites() {
        println!("{}", item);
    }
}
