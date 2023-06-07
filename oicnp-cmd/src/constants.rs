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
            dir: "blog/backend/_posts",
            weight: 10,
            parent: "cms",
        },
        Category {
            name: "frontend",
            dir: "blog/frontend/_posts",
            weight: 20,
            parent: "cms",
        },
        Category {
            name: "rust",
            dir: "blog/rust/_posts",
            weight: 30,
            parent: "cms",
        },
        Category {
            name: "server",
            dir: "blog/server/_posts",
            weight: 40,
            parent: "cms",
        },
        Category {
            name: "diary",
            dir: "blog/diary/_posts",
            weight: 50,
            parent: "cms",
        },
    ]
}
