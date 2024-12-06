mod macos_impl;
pub mod macos;

mod repository;
mod sidebar;
mod sidebar_item;
mod target;

pub use repository::SidebarRepository;
pub use sidebar::Sidebar;
pub use sidebar_item::SidebarItem;
pub use target::Target;

pub struct Finder {
    sidebar: Sidebar,
}

impl Finder {
    pub fn new<T: macos::MacOsApi>(repo: SidebarRepository<T>) -> Self {
        let sidebar = repo.load().unwrap_or_default();
        Self { sidebar }
    }

    pub fn sidebar(&self) -> &Sidebar {
        &self.sidebar
    }
}
