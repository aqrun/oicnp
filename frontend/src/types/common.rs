
pub enum MenuId {
    Home = 100,
    Backend = 200,
    Frontend = 300,
    Server = 400,
    Rust = 500,
    Diary = 600,
}

pub struct MenuItem {
    pub id: MenuId,
    pub name: String,
    pub href: String,
    pub vid: String,
}
