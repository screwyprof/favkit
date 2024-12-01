use clap::{Parser, Subcommand};
use favkit::sidebar::{Result, Sidebar, SidebarOperations, SidebarSection};
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all sidebar items
    List {
        #[arg(short, long)]
        section: Option<String>,
    },
    /// Add an item to the sidebar
    Add {
        /// Path to add to the sidebar
        path: String,
    },
    /// Remove an item from the sidebar
    Remove {
        /// Path to remove from the sidebar
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { section } => {
            let section = match section.as_deref() {
                Some(s) => SidebarSection::from_str(s)?,
                None => SidebarSection::Favorites,
            };

            let sidebar = Sidebar::new(section)?;

            match section {
                SidebarSection::Favorites => println!("\nFavorites:"),
                SidebarSection::Locations => println!("\nLocations:"),
            }

            for item in sidebar.list_items()? {
                println!("{} -> {}", item.name, item.url);
            }
        }
        Commands::Add { path } => {
            let sidebar = Sidebar::favorites()?;
            sidebar.add_item(&path)?;
            println!("Added item: {}", path);
        }
        Commands::Remove { path } => {
            let sidebar = Sidebar::favorites()?;
            sidebar.remove_item(&path)?;
            println!("Removed item: {}", path);
        }
    }

    Ok(())
}
