use crate::models::Category;
use std::sync::OnceLock;

pub static CATEGORIES: OnceLock<Vec<Category>> = OnceLock::new();

pub fn init_categories() -> Vec<Category<'static>> {
    vec![
        Category {
            name: "cms",
            dir: "",
            weight: 1,
            parent: "",
        },
        Category {
            name: "backend",
            dir: "content/backend",
            weight: 10,
            parent: "cms",
        },
        Category {
            name: "frontend",
            dir: "content/frontend",
            weight: 20,
            parent: "cms",
        },
        Category {
            name: "rust",
            dir: "content/rust",
            weight: 30,
            parent: "cms",
        },
        Category {
            name: "server",
            dir: "content/server",
            weight: 40,
            parent: "cms",
        },
        Category {
            name: "diary",
            dir: "content/diary",
            weight: 50,
            parent: "cms",
        },
    ]
}
