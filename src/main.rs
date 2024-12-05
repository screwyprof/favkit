#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use favkit::errors::Result;

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() -> Result<()> {
    Ok(())
}
