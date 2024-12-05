mod favorites;
mod sidebar;
mod sidebar_item;
mod target;

pub use favorites::Favorites;
pub use sidebar::Sidebar;
pub use sidebar_item::SidebarItem;
pub use target::Target;

#[derive(Default)]
pub struct Finder {
    sidebar: Sidebar,
}

impl Finder {
    pub fn sidebar(&self) -> &Sidebar {
        &self.sidebar
    }
}
