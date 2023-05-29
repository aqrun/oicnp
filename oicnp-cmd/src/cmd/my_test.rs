use oicnp_core::{
    DB, establish_connection,
    services::{
        find_nodes_count, find_taxonomy_by_vid,
    },
    typings::{NodeBundle},
};

pub async fn run() {
    println!("Test run----");
    let db = DB.get_or_init(establish_connection).await;
    let bundle = NodeBundle::Article.to_string();
    let category = "rust";

    // let count = find_nodes_count(db, &bundle, category).await;
    let res = find_taxonomy_by_vid(db, category).await;
    println!("res------ {:?}", res);
}