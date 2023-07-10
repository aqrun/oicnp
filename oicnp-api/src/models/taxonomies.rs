use async_graphql::{Object, Context};
use oicnp_core::{
    models::{
        Taxonomies as CoreTaxonomies, Tag as CoreTag,
    },
};

#[derive(Clone, Debug)]
pub struct Taxonomies {
    pub tid: String,
    pub vid: String,
    pub pid: String,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
}

#[Object]
impl Taxonomies {
    async fn tid(&self) -> &str {
        self.tid.as_str()
    }
    async fn vid(&self) -> &str {
        self.vid.as_str()
    }
    async fn pid(&self) -> &str {
        self.pid.as_str()
    }
    async fn name(&self) -> &str {
        self.name.as_str()
    }
    async fn description(&self) -> &str {
        self.vid.as_str()
    }
    async fn description_format(&self) -> &str {
        self.vid.as_str()
    }
    async fn weight(&self) -> i32 {
        self.weight
    }
}

impl From<&CoreTaxonomies> for Taxonomies {
    fn from(t: &CoreTaxonomies) -> Self {
        Self {
            tid: String::from(&t.tid),
            vid: String::from(&t.vid),
            pid: String::from(&t.pid),
            name: String::from(&t.name),
            description: String::from(&t.description),
            description_format: String::from(&t.description_format),
            weight: t.weight,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NewTaxonomy {
    pub vid: String,
    pub pid: i32,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub data: CoreTag,
}

#[Object]
impl Tag {
    async fn id(&self) -> &str {
        self.data.tag_id.as_str()
    }
    async fn vid(&self) -> &str {
        self.data.vid.as_str()
    }
    async fn name(&self) -> &str {
        self.data.name.as_str()
    }
    async fn weight(&self) -> i32 {
        self.data.weight
    }
    async fn count(&self) -> i64 {
        self.data.count
    }
}
