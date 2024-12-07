#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::finder::{
    Finder,
    macos_impl::SystemMacOsApi,
    repository::Repository,
};

mod finder;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    let api = SystemMacOsApi::new();
    let repository = Repository::new(Box::new(api));
    let sidebar = repository.load();
    let finder = Finder::new(sidebar);

    println!("Finder Sidebar Items:");
    println!("--------------------");
    for item in finder.sidebar().favorites() {
        println!("{}", item);
    }
}
