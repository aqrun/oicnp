use oicnp_core::{
    prelude::{
        sea_orm::*,
        serde::{Serialize, Deserialize},
    },
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
    utils::{get_config_file_path,
            slugify_paths_without_date, capture_file_name,
    },
    models::{
        Node, NodeBody,
    }
};

pub async fn run() {
    println!("Test run----");
    let db = DB.get_or_init(establish_connection).await;
    let bundle = NodeBundle::Article.to_string();
    let category = "rust";

    // let count = find_nodes_count(db, &bundle, category).await;
    // let res = find_taxonomy_by_vid(db, category).await;
    // println!("res------ {:?}", res);
    // find_node_with_body(db).await;
    // find_nodes_by_taxonomy(db).await;
    // get_config_path();
    get_slug_url();
}

/// slug 函数测试
fn get_slug_url() {
    let s = "2014-02-24-php代码展示";
    let slug = slugify_paths_without_date(s);

    let s1 = "2014-02-24-php代码展示.md";
    let res1 = slugify_paths_without_date(s1);

    let s2 = "content\\diary\\2017-10-07-程序员提高方式.md";
    let res2 = slugify_paths_without_date(s2);
    let s3 = "content/diary/2017-10-07-程序员提高方式.md";
    let res3 = slugify_paths_without_date(s3);

    println!("{}: {}", s, slug);
    println!("{}: {}", s1, res1);
    println!("{}: {}", s2, res2);
    println!("{}: {}", s3, res3);
}

pub fn get_config_path() {
    let p = get_config_file_path();
    println!("file path is: {}", p);
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