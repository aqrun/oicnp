use oicnp_core::{
    entities::{
        cms_nodes, cms_taxonomies,
        prelude::{CmsNodes, CmsTaxonomies},
    },
    establish_connection,
    prelude::sea_orm::*,
    services::{find_node_taxonomies, find_nodes},
    typings::NodeBundle,
    utils::{
        get_config_file_path, slugify_paths_without_date, uuid,
    },
    DatabaseConnection, DB,
};
use migration::types::CmsFiles;

pub async fn run() {
    println!("Test run----");
    // let db = DB.get_or_init(establish_connection).await;
    // let bundle = NodeBundle::Article.to_string();
    // let category = "rust";

    // let count = find_nodes_count(db, &bundle, category).await;
    // let res = find_taxonomy_by_vid(db, category).await;
    // println!("res------ {:?}", res);
    // find_node_with_body(db).await;
    // find_nodes_by_taxonomy(db).await;
    // get_config_path();
    // get_slug_url();
    // let a = youdao_translate("中国人").await;
    // get_node_taxonomies(db).await;
    //
    // column_from_str();
    // generate_uuid();
    derive_test();
}

fn derive_test() {
    let file_name = CmsFiles::table_name("oic_");
    println!("file____  {:?} --- {}", file_name.to_string(), CmsFiles::Table.to_string());
}

fn generate_uuid() {
    // let mut bucket = SnowflakeIdGenerator::new(2, 3);
    // bucket.real_time_generate()

    for _ in 0..30 {
        println!("{:?}", uuid());
    }
}

fn column_from_str() {
    let a = cms_nodes::Column::try_from("created_at").unwrap();

    match a {
        cms_nodes::Column::CreatedAt => {
            println!("matched createdat {:?}", a);
        }
        _ => {
            println!("nothing match {:?}", a);
        }
    };
}

async fn get_node_taxonomies(db: &DbConn) {
    let a = find_node_taxonomies(db, "1hss6so1js8ac").await;
}

/// slug 函数测试
fn get_slug_url() {
    let s = "2014-02-24-php代码展示";
    let (_, slug) = slugify_paths_without_date(s);

    let s1 = "2014-02-24-php代码展示.md";
    let (_, res1) = slugify_paths_without_date(s1);

    let s2 = "content\\diary\\2017-10-07-程序员提高方式.md";
    let (_, res2) = slugify_paths_without_date(s2);
    let s3 = "content/diary/2017-10-07-程序员提高方式.md";
    let (_, res3) = slugify_paths_without_date(s3);

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
    /*
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
     */
    let bundle = "article";
    let category = "backend";
    let filters = Vec::new();
    let order_name = "";
    let order_dir = "desc";
    let offset = 0;
    let limit = 2;

    match find_nodes(
        db, bundle, category, &filters, order_name, order_dir, offset, limit,
    )
    .await
    {
        Ok(a) => println!("success, {:?}", a),
        Err(err) => println!("failed: {:?}", err),
    }
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
        .filter(cms_taxonomies::Column::Name.eq("rust"));

    let paginator = q.paginate(db, page_size);
    let total_page = paginator.num_pages().await.unwrap();
    let data = paginator.fetch_page(page - 1).await;
    let data = data.unwrap();
    println!("data----{:?}", data);
    println!("total: {}", total_page);
}
