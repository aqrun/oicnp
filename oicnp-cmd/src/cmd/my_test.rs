use oicnp_core::{
    DB, establish_connection, DatabaseConnection,
    services::{
        find_nodes_count, find_taxonomy_by_vid,
    },
    typings::{NodeBundle},
    entities::{
        prelude::{
            CmsNodes, CmsNodeBody, CmsTaxonomies,
        },
        cms_nodes, cms_node_body, cms_taxonomies,
    },
    models::{
        Node, NodeBody,
    }
};
use sea_orm::*;
use serde::{Serialize, Deserialize};

pub async fn run() {
    println!("Test run----");
    let db = DB.get_or_init(establish_connection).await;
    let bundle = NodeBundle::Article.to_string();
    let category = "rust";

    // let count = find_nodes_count(db, &bundle, category).await;
    // let res = find_taxonomy_by_vid(db, category).await;
    // println!("res------ {:?}", res);
    // find_node_with_body(db).await;
    find_nodes_by_taxonomy(db).await;
}

// one to one 查找node 和 node_body
pub async fn find_node_with_body(db: &DatabaseConnection) {
    let a = CmsNodes::find()
        .find_also_related(CmsNodeBody)
        // .select_only()
        // .column(cms_nodes::Column::Nid)
        // .column(cms_nodes::Column::Vid)
        // .column(cms_nodes::Column::Title)
        // .column(cms_node_body::Column::Summary)
        ;

    let sql = a.build(DbBackend::Postgres)
        .to_string();


    let data = // a.into_model::<Node, NodeBody>()
        a.one(db)
        .await;

    println!("sql-----{:?}", sql);
    println!("data-----{:?}", data);
}

// 按分类查找
pub async fn find_nodes_by_taxonomy(db: &DatabaseConnection) {
    let page_size = 2;
    let page = 2;
    let q = CmsNodes::find()
        .find_also_related(CmsTaxonomies)
        // .select_only()
        // .column(cms_nodes::Column::Nid)
        // .column(cms_nodes::Column::Title)
        // .column(cms_taxonomies::Column::Name)
        .filter(cms_taxonomies::Column::Name.eq("rust"))
        ;

    let paginator = q.paginate(db, page_size);
    let total_page = paginator.num_pages().await.unwrap();
    let data = paginator.fetch_page(page - 1)
        .await;
    let data = data.unwrap();
    println!("data----{:?}", data);
    println!("total: {}", total_page);
}