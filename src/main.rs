use favkit::Sidebar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sidebar = Sidebar::new();
    sidebar.favorites().list_items();

    Ok(())
}
