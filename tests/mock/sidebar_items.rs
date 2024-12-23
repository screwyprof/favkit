use favkit::finder::{SidebarItem, Target};

pub struct SidebarItemsBuilder(Vec<SidebarItem>);

impl SidebarItemsBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn airdrop(mut self) -> Self {
        self.0.push(SidebarItem::new(Target::AirDrop));
        self
    }
    pub fn recents(mut self) -> Self {
        self.0.push(SidebarItem::new(Target::Recents));
        self
    }

    pub fn applications(mut self) -> Self {
        self.0.push(SidebarItem::new(Target::Applications));
        self
    }

    pub fn custom(mut self, label: &str, path: &str) -> Self {
        self.0.push(SidebarItem::new(Target::Custom {
            label: label.to_string(),
            path: path.to_string(),
        }));
        self
    }

    pub fn build(self) -> Vec<SidebarItem> {
        self.0
    }
}
