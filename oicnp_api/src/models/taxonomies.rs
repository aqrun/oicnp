use async_graphql::{Object, Context};

#[crud_table]
#[derive(Clone, Debug)]
pub struct Taxonomies {
    pub tid: i32,
    pub vid: String,
    pub pid: i32,
    pub bundle: String,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
    pub count: i32,
}

#[Object]
impl Taxonomies {
    async fn tid(&self) -> i32 {
        self.tid
    }
    async fn vid(&self) -> &str {
        self.vid.as_str()
    }
    async fn pid(&self) -> i32 {
        self.pid
    }
    async fn bundle(&self) -> &str {
        self.bundle.as_str()
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
    async fn count(&self) -> i32 {
        self.count
    }
}

#[crud_table(table_name: taxonomies)]
#[derive(Clone, Debug)]
pub struct NewTaxonomy {
    pub vid: String,
    pub pid: i32,
    pub bundle: String,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
}
