use favkit::Finder;

#[cfg_attr(coverage, coverage(off))]
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
