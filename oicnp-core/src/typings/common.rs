use strum_macros::Display;
use serde::{Deserialize, Serialize};

// #[cfg(target_os = "windows")]
// pub type oic_usize = u64;
//
// #[cfg(not(target_os = "windows"))]
// pub type oic_usize = u64;

#[derive(Display, Debug)]
pub enum TaxonomyBundle {
    #[strum(serialize = "category")]
    Category,
    #[strum(serialize = "tag")]
    Tag,
}

#[derive(Display, Debug)]
pub enum NodeBundle {
    #[strum(serialize = "article")]
    Article,
}

impl From<&str> for NodeBundle {
    fn from(bundle: &str) -> Self {
        match bundle {
            "article" => Self::Article,
            _ => Self::Article,
        }
    }
}

#[derive(Display, Debug)]
pub enum BodyFormat {
    #[strum(serialize = "markdown")]
    Markdown,
    #[strum(serialize = "html")]
    Html,
    #[strum(serialize = "text")]
    Text,
}

#[derive(Display, Debug)]
pub enum UserPicturesBundle {
    #[strum(serialize = "avatar")]
    Avatar,
    #[strum(serialize = "image")]
    Image,
}

#[derive(Deserialize, Serialize)]
pub struct Count {
    pub count: i32,
}
