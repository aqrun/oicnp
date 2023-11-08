
pub enum MenuId {
    Home = 100,
    Backend = 200,
    Frontend = 300,
    Server = 400,
    Rust = 500,
    Diary = 600,
}

impl MenuId {
    pub fn get_vid(&self) -> &str {
        match self {
            Self::Home => "home",
            Self::Backend => "backend",
            Self::Frontend => "frontend",
            Self::Server => "server",
            Self::Rust => "rust",
            Self::Diary => "diary",
            _ => "home",
        }
    }
}

pub struct MenuItem {
    pub id: MenuId,
    pub name: &'static str,
    pub href: &'static str,
    pub vid: &'static str,
}
