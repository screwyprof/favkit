use clap::{Parser, Subcommand};
use favkit::sidebar::{Result, Sidebar, SidebarSection};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List items in the sidebar
    List {
        /// Section to list (favorites or locations)
        #[arg(short, long)]
        section: SidebarSection,
    },
    /// Add an item to favorites
    Add {
        /// Path to add
        path: String,
    },
    /// Remove an item from favorites
    Remove {
        /// Item name to remove
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let sidebar = Sidebar::new();

    match cli.command {
        Commands::List { section } => {
            let items = match section {
                SidebarSection::Favorites => sidebar.favorites().list_items()?,
                SidebarSection::Locations => sidebar.locations().list_items()?,
            };

            for item in items {
                println!("{}: {}", item.name, item.url);
            }
        }
        Commands::Add { path } => {
            sidebar.favorites().add_item(path)?;
            println!("Item added successfully");
        }
        Commands::Remove { name } => {
            sidebar.favorites().remove_item(&name)?;
            println!("Item removed successfully");
        }
    }

    Ok(())
}
