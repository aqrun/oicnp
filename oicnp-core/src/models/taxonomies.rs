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