
#[derive(Queryable)]
pub struct Taxonomy {
    pub tid: i32,
    pub vid: i32,
    pub bundle: String,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
}

#[derive(Insertable)]
#[table_name = "taxonomy"]
pub struct NewTaxonomy {
    pub vid: i32,
    pub bundle: String,
    pub name: String,
    pub description: String,
    pub description_format: String,
    pub weight: i32,
}