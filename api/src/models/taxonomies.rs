// use crate::schema::taxonomy;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Taxonomy {
    pub tid: Option<i32>,
    pub vid: Option<String>,
    pub pid: Option<i32>,
    pub bundle: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub description_format: Option<String>,
    pub weight: Option<i32>,
}
