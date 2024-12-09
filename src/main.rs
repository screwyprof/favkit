#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod errors;

use errors::Result;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<()> {
    println!("Hello World!");

    Ok(())
}