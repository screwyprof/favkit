use crate::errors::FinderError;
use crate::finder::sidebar::item::SidebarItem;
use crate::finder::sidebar::target::Target;
use crate::finder::system::api::{MacOsApi, SidebarItemRef};

/// Repository is responsible for loading and saving sidebar items.
pub struct Repository<A: MacOsApi> {
    api: A,
}

impl<A: MacOsApi> Repository<A> {
    pub fn new(api: A) -> Self {
        Self { api }
    }

    /// Load all sidebar items from the favorites list
    pub fn load(&self) -> Result<Vec<SidebarItem>, FinderError> {
        let mut items = Vec::new();
        let mut seed = 0;

        // SAFETY: We ensure that the list is properly released when no longer needed
        unsafe {
            let list = self.api.get_favorites_list()?;
            let snapshot = self.api.get_favorites_snapshot(list, &mut seed)?;

            for item_ref in snapshot.iter() {
                if let Some(item) = self.process_item(item_ref) {
                    items.push(item);
                }
            }
        }

        Ok(items)
    }

    /// Process a single item from the favorites list
    unsafe fn process_item(&self, item_ref: SidebarItemRef) -> Option<SidebarItem> {
        let url = self.api.get_item_url(item_ref)?;
        let display_name = self.api.get_item_display_name(item_ref);

        let target = Target::try_from(url).ok()?;

        match target {
            Target::AirDrop(_) => Some(SidebarItem::new(target, "AirDrop")),
            _ => display_name
                .filter(|name| !name.is_empty())
                .map(|name| SidebarItem::new(target, name)),
        }
    }
}
