use crate::errors::FinderError;
use crate::finder::sidebar::item::SidebarItem;
use crate::finder::sidebar::target::Target;
use crate::finder::system::api::{MacOsApi, SidebarItemRef};

/// Repository is responsible for loading and saving sidebar items.
///
/// # Examples
///
/// ```
/// use favkit::finder::repository::Repository;
/// use favkit::finder::system::api::RealMacOsApi;
///
/// let repo = Repository::new(RealMacOsApi::new());
/// match repo.load() {
///     Ok(items) => println!("Loaded {} items", items.len()),
///     Err(e) => eprintln!("Failed to load items: {}", e),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Repository<A: MacOsApi> {
    api: A,
}

impl<A: MacOsApi> Repository<A> {
    /// Creates a new repository with the given API implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use favkit::finder::repository::Repository;
    /// use favkit::finder::system::api::RealMacOsApi;
    ///
    /// let repo = Repository::new(RealMacOsApi::new());
    /// ```
    #[must_use]
    pub fn new(api: A) -> Self {
        Self { api }
    }

    /// Loads all sidebar items from the favorites list.
    ///
    /// # Errors
    ///
    /// Returns `FinderError` if:
    /// - Failed to get the favorites list
    /// - Failed to get a snapshot of the favorites list
    ///
    /// # Examples
    ///
    /// ```
    /// use favkit::finder::repository::Repository;
    /// use favkit::finder::system::api::RealMacOsApi;
    ///
    /// let repo = Repository::new(RealMacOsApi::new());
    /// let items = repo.load().expect("Failed to load sidebar items");
    /// println!("Loaded {} items", items.len());
    /// ```
    pub fn load(&self) -> Result<Vec<SidebarItem>, FinderError> {
        // SAFETY: We ensure that the list is properly released when no longer needed
        unsafe {
            let list = self.api.get_favorites_list()?;
            let snapshot = self.api.get_favorites_snapshot(&list, &mut 0)?;

            Ok(snapshot
                .iter()
                .filter_map(|item_ref| self.process_item(item_ref))
                .collect())
        }
    }

    /// Processes a single item from the favorites list.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it interacts with macOS APIs that require unsafe blocks.
    /// The caller must ensure that:
    /// - The item_ref is valid and points to a valid LSSharedFileListItemRef
    /// - The item_ref remains valid for the duration of this call
    ///
    /// # Returns
    ///
    /// Returns `None` if:
    /// - Failed to get the item's URL
    /// - Failed to convert the URL to a Target
    /// - The item has an empty display name (except for AirDrop)
    unsafe fn process_item(&self, item_ref: SidebarItemRef) -> Option<SidebarItem> {
        // Get the display name first to match the expected order of API calls
        let display_name = match self.api.get_item_display_name(item_ref) {
            Some(name) if !name.is_empty() => name,
            Some(_) => "AirDrop".to_string(), // Empty name might be AirDrop
            None => return None,
        };

        // Get the URL and try to convert it to a target
        let url = self.api.get_item_url(item_ref)?;
        let url_str = String::from(&url);
        
        // If it's not AirDrop and has an empty name, return None
        if display_name == "AirDrop" && !url_str.starts_with("nwnode://") {
            return None;
        }
        
        // Try to convert the URL to a target, return None if invalid
        match Target::try_from(&url) {
            Ok(target) => Some(SidebarItem::new(target, display_name)),
            Err(_) => None,
        }
    }
}
