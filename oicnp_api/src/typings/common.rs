use strum_macros::Display;

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