use favkit::{Finder, Repository, SystemMacOsApi};

#[test]
fn test_load_favorites() -> Result<(), Box<dyn std::error::Error>> {
    let api = Box::new(SystemMacOsApi::new());
    let repository = Repository::new(api);
    let sidebar = repository.load()?;
    let finder = Finder::new(sidebar);
    
    println!("{}", finder);
    Ok(())
}
