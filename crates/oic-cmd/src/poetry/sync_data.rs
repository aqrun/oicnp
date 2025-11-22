use anyhow::Result;
use super::{sync_authors_data, sync_poetry_data, sync_books_data, sync_yuding_quan_tang_shi};
use crate::utils::format_duration;
use oic_core::models::poetry::PoetryModel;
use super::{DB, db_conn};

pub async fn sync_data() -> Result<()> {
    // 记录开始时间
    let start_time = std::time::Instant::now();
    let db = DB.get_or_init(db_conn).await;

    let poetry_dir = dotenv::var("POETRY_DIR").expect("POETRY_DIR 环境变量未设置");
    sync_authors_data(poetry_dir.as_str()).await?;
    sync_poetry_data(poetry_dir.as_str()).await?;
    sync_yuding_quan_tang_shi(poetry_dir.as_str()).await?;
    sync_books_data(poetry_dir.as_str()).await?;

    // 记录结束时间 打印秒级时间
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("\n\n全部数据总耗时: {}", format_duration(duration));
    println!("{}", PoetryModel::get_analysis_view(db).await?);
    Ok(())
}
