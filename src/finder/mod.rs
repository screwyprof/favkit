mod favorites;
mod sidebar;
mod sidebar_item;
mod target;
mod repository;

pub use favorites::Favorites;
pub use sidebar::Sidebar;
pub use sidebar_item::SidebarItem;
pub use target::Target;
pub use repository::SidebarRepository;

use crate::errors::Result;

pub struct Finder {
    sidebar: Sidebar,
    repository: Box<dyn SidebarRepository>,
}

impl Finder {
    pub fn new(repository: Box<dyn SidebarRepository>) -> Self {
        Self {
            sidebar: Sidebar::default(),
            repository,
        }
    }

    pub fn start(repository: Box<dyn SidebarRepository>) -> Result<Self> {
        let sidebar = repository.load()?;
        Ok(Self { sidebar, repository })
    }

    pub fn quit(&self) -> Result<()> {
        self.repository.save(&self.sidebar)
    }

    pub fn sidebar(&self) -> &Sidebar {
        &self.sidebar
    }

    pub fn sidebar_mut(&mut self) -> &mut Sidebar {
        &mut self.sidebar
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    struct TestRepository;
    impl SidebarRepository for TestRepository {
        fn load(&self) -> Result<Sidebar> {
            Ok(Sidebar::default())
        }
        fn save(&self, _: &Sidebar) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn finder_provides_access_to_sidebar() {
        let finder = Finder::new(Box::new(TestRepository));
        assert!(finder.sidebar().favorites().items().is_empty());
    }
}
