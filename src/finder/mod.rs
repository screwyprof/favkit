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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn finder_provides_access_to_sidebar() {
        let finder = Finder::default();
        assert!(finder.sidebar().favorites().items().is_empty());
    }
}
