use crate::models::Category;

pub fn get_categories() -> Vec<Category<'static>> {
    vec![
        Category { name: "backend", dir: "blog/backend/_posts", weight: 10 },
        Category { name: "frontend", dir: "blog/frontend/_posts", weight: 20 },
        Category { name: "rust", dir: "blog/rust/_posts", weight: 30 },
        Category { name: "server", dir: "blog/server/_posts", weight: 40 },
        Category { name: "diary", dir: "blog/diary/_posts", weight: 50 },
    ]
}
