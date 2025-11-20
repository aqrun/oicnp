use anyhow::Result;
use std::fs;
use super::models::WudaiPoetryModel;
use oic_core::{
    entities::poetry::*,
    uuid,
    models::ModelCrudHandler,
    models::poetry::*,
};
use super::db::{db_conn, DB};

pub async fn sync_data() -> Result<()> {
    let poetry_dir = dotenv::var("POETRY_DIR").expect("POETRY_DIR 环境变量未设置");
    sync_wudai_data(poetry_dir.as_str()).await?;
    Ok(())
}

///
/// 同步五代诗词数据
/// 
/// 1 花间集
/// 目录: "五代诗词/huajianji/", 
/// 朝代: 五代
/// 标签: 花间集
/// 处理方式： 遍历目录下json文件读取文件，解析 json 数据，插入数据库，并插入作者数据
/// 
/// 2 南唐
/// 文件: "五代诗词/nantang/poetrys.json", 
/// 朝代: 五代
/// 标签：南唐
/// 处理方式： 读取文件，解析 json 数据，插入诗词数据，并插入作者数据
pub async fn sync_wudai_data(poetry_dir: &str) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let path = format!("{}/五代诗词/huajianji/", poetry_dir);
    let files = fs::read_dir(path)?;
    for file in files {
        let file = file?;
        let path = file.path();

        if path.ends_with("README.md") {
            continue;
        }

        let content = fs::read_to_string(path.clone())?;
        let poetry_list: Vec<WudaiPoetryModel> = match serde_json::from_str(&content) {
            Ok(x) => x,
            Err(e) => {
                return Err(anyhow::anyhow!("解析文件数据失败: {:?} - {}", path.display(), e));
            }
        };

        let mut i = 0;
        for item in poetry_list {
            i += 1;
            let author = CreateAuthorReqParams {
                uuid: Some(uuid!()),
                name: Some(String::from(item.author.as_str())),
                dynasty: Some(String::from("五代")),
                ..Default::default()
            };
            let author_id = AuthorModel::upsert(db, &author).await?;

            let tags: Vec<String> = vec![
                String::from("花间集"),
                String::from(item.rhythmic.as_str())
            ];
            let content: String = item.paragraphs.join("\n");
            let poetry = CreatePoetryReqParams {
                uuid: Some(uuid!()),
                title: Some(String::from(item.title.as_str())),
                author_id: Some(author_id as i32),
                dynasty: Some(String::from("五代")),
                tags: Some(tags.join(",")),
                content: Some(content.clone()),
                word_count: Some(content.len() as i32),
                weight: Some(i),
                ..Default::default()
            };
            let _ = PoetryModel::create(db, &poetry).await?;
        }
        println!("花间集诗词个数: {}", i);
    }
    Ok(())
}