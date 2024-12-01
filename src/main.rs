use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::{CFString, CFStringRef},
    url::{CFURLGetString, CFURL},
};
use core_services::{
    kLSSharedFileListFavoriteItems, kLSSharedFileListFavoriteVolumes,
    kLSSharedFileListSessionLoginItems, LSSharedFileListCopySnapshot, LSSharedFileListCreate,
    LSSharedFileListInsertItemURL, LSSharedFileListItemCopyDisplayName,
    LSSharedFileListItemCopyResolvedURL, LSSharedFileListItemRef, LSSharedFileListItemRemove,
    LSSharedFileListRef,
};

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
        Commands::List { section } => match section.as_deref() {
            Some("favorites") | None => {
                println!("\nFavorites:");
                unsafe { list_section(kLSSharedFileListFavoriteItems)? };
            }
            Some("locations") => {
                println!("\nLocations:");
                unsafe { list_section(kLSSharedFileListFavoriteVolumes)? };
            }
            Some("login") => {
                println!("\nLogin Items:");
                unsafe { list_section(kLSSharedFileListSessionLoginItems)? };
            }
            Some(unknown) => {
                println!("Unknown section: {}", unknown);
                println!("Available sections: favorites, locations, login");
            }
        },
        Commands::Add { path } => {
            let list = unsafe { create_list(kLSSharedFileListFavoriteItems)? };
            add_item(list, &path)?;
        }
        Commands::Remove { path } => {
            let list = unsafe { create_list(kLSSharedFileListFavoriteItems)? };
            remove_item(list, &path)?;
        }
    }

    Ok(())
}

unsafe fn create_list(list_type: CFStringRef) -> Result<LSSharedFileListRef> {
    let list = LSSharedFileListCreate(std::ptr::null(), list_type, std::ptr::null());
    if list.is_null() {
        anyhow::bail!("Failed to create list");
    }
    Ok(list)
}

unsafe fn list_section(section_type: CFStringRef) -> Result<()> {
    let list = create_list(section_type)?;
    list_items(list)
}

fn list_items(list: LSSharedFileListRef) -> Result<()> {
    unsafe {
        let mut seed: u32 = 0;
        let items_ptr = LSSharedFileListCopySnapshot(list, &mut seed);
        if items_ptr.is_null() {
            anyhow::bail!("Failed to get items snapshot");
        }

        let items: CFArray<CFType> = CFArray::wrap_under_create_rule(items_ptr.cast());
        for item in items.iter() {
            let item_ref = item.as_concrete_TypeRef() as LSSharedFileListItemRef;

            // Get the display name
            let name_ref = LSSharedFileListItemCopyDisplayName(item_ref);
            let name = if !name_ref.is_null() {
                let cf_name = CFString::wrap_under_create_rule(name_ref);
                cf_name.to_string()
            } else {
                String::from("")
            };

            // Get the URL
            let url_ref = LSSharedFileListItemCopyResolvedURL(item_ref, 0, std::ptr::null_mut());
            if !url_ref.is_null() {
                let url = CFURL::wrap_under_create_rule(url_ref);
                let url_str_ref = url.as_concrete_TypeRef();
                let url_str = CFString::wrap_under_create_rule(CFURLGetString(url_str_ref).cast());
                let url_string = url_str.to_string();

                // Check for AirDrop using the URL scheme
                if url_string.starts_with("nwnode://")
                    || (name.is_empty() && url_string.contains("AirDrop"))
                {
                    println!(" -> nwnode://domain-AirDrop");
                    continue;
                }

                if let Some(path) = url.to_path() {
                    // Add trailing slash for directories
                    let path_str = path.display().to_string();
                    let path_with_slash = if path.is_dir() && !path_str.ends_with('/') {
                        format!("{}/", path_str)
                    } else {
                        path_str
                    };
                    println!("{} -> file://{}", name, path_with_slash);
                } else {
                    // For other non-file URLs
                    println!("{} -> {}", name, url_str);
                }
            } else {
                println!("{} -> NOTFOUND", name);
            }
        }
    }
    Ok(())
}

fn add_item(list: LSSharedFileListRef, path: &str) -> Result<()> {
    let url = CFURL::from_path(path, true)
        .with_context(|| format!("Failed to create URL from path: {}", path))?;

    unsafe {
        LSSharedFileListInsertItemURL(
            list,
            std::ptr::null_mut(), // Insert at end
            std::ptr::null_mut(), // No display name (use default)
            std::ptr::null_mut(), // No icon
            url.as_concrete_TypeRef(),
            std::ptr::null(),     // No properties
            std::ptr::null_mut(), // No properties
        );
    }

    println!("Added item: {}", path);
    Ok(())
}

fn remove_item(list: LSSharedFileListRef, path: &str) -> Result<()> {
    unsafe {
        let mut seed: u32 = 0;
        let items_ptr = LSSharedFileListCopySnapshot(list, &mut seed);
        if items_ptr.is_null() {
            anyhow::bail!("Failed to get items snapshot");
        }

        let target_path = std::path::PathBuf::from(path);
        let items: CFArray<CFType> = CFArray::wrap_under_create_rule(items_ptr.cast());
        for item in items.iter() {
            let item_ref = item.as_concrete_TypeRef() as LSSharedFileListItemRef;

            // Get the URL for the item
            let url_ref = LSSharedFileListItemCopyResolvedURL(item_ref, 0, std::ptr::null_mut());
            if !url_ref.is_null() {
                let url = CFURL::wrap_under_create_rule(url_ref);
                if let Some(item_path) = url.to_path() {
                    if item_path == target_path {
                        LSSharedFileListItemRemove(list, item_ref);
                        println!("Removed item: {}", path);
                        return Ok(());
                    }
                }
            }
        }
        anyhow::bail!("Item not found: {}", path);
    }
}
