
#[crud_table]
#[derive(Clone, Debug)]
pub struct Taxonomy {
    pub tid: i32,
    pub vid: String,
    pub pid: i32,
    pub bundle: String,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
}

#[crud_table(table_name: taxonomy)]
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
