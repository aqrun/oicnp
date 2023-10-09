use oicnp_core::{
    prelude::{
        strum,
        strum_macros::Display,
    },
};
use serde::{Serialize, Deserialize};

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

#[derive(Display, Debug)]
pub enum TaxonomyBundle {
    #[strum(serialize = "category")]
    Category,
    #[strum(serialize = "tag")]
    Tag,
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

#[derive(Debug)]
pub struct Token(pub String);

