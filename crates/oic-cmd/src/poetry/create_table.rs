///
/// 使用 sea-orm-migration 创建表
///
use anyhow::Result;
use sea_orm_migration::{prelude::*, schema::*};
use super::db::{db_conn, DB};

#[derive(DeriveIden)]
#[allow(dead_code)]
enum Authors {
    Table,
    Id,
    Uuid,
    Name,
    Description,
    ShortDescription,
    BirthAt,
    DeathAt,
    Dynasty,
    Weight,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Poetry {
    Table,
    Id,
    Uuid,
    Title,
    AuthorId,
    Dynasty,
    Weight,
    HotWeight,
    Content,
    WordCount,
    Tags,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum PoetryLines {
    Table,
    Id,
    PoetryId,
    LineNumber,
    Content,
    Pinyin,
    Description,
    Notes,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Chapters {
    Table,
    Id,
    Uuid,
    Pid,
    PoetryId,
    Title,
    /// 章节总内容
    Content,
    WordCount,
    Weight,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ChapterLines {
    Table,
    Id,
    ChapterId,
    LineNumber,
    Content,
    Pinyin,
    Description,
    Notes,
    CreatedAt,
    UpdatedAt,
}

/// 部分诗集信息说明表
#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    Name,
    Description,
}

pub async fn create_tables() -> Result<()> {
    delete_tables().await?;
    
    let db = DB.get_or_init(db_conn).await;
    let manager = SchemaManager::new(db);
    
    // 创建 authors 表
    manager
        .create_table(
            Table::create()
                .table(Authors::Table)
                .if_not_exists()
                .col(pk_auto(Authors::Id))
                .col(string_len_uniq(Authors::Uuid, 32))
                .col(string_len_uniq(Authors::Name, 100))
                .col(text(Authors::Description).default(""))
                .col(text(Authors::ShortDescription).default(""))
                .col(date_time_null(Authors::BirthAt))
                .col(date_time_null(Authors::DeathAt))
                .col(string_len(Authors::Dynasty, 20).default(""))
                .col(small_integer(Authors::Weight).default(0))
                .col(date_time(Authors::CreatedAt).default(Expr::current_timestamp()))
                .col(date_time_null(Authors::UpdatedAt).default(Expr::current_timestamp()))
                .to_owned(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("创建 authors 表失败: {}", e))?;

    println!("authors 表创建成功");

    // 创建 poetry 表
    manager
        .create_table(
            Table::create()
                .table(Poetry::Table)
                .if_not_exists()
                .col(pk_auto(Poetry::Id))
                .col(string_len_uniq(Poetry::Uuid, 32))
                .col(string_len(Poetry::Title, 255))
                .col(integer(Poetry::AuthorId).default(0))
                .col(string_len(Poetry::Dynasty, 20).default(""))
                .col(integer(Poetry::Weight).default(0))
                .col(small_integer(Poetry::HotWeight).default(0))
                .col(text(Poetry::Content))
                .col(small_integer(Poetry::WordCount).default(0))
                .col(string_len(Poetry::Tags, 255).default(""))
                .col(date_time(Poetry::CreatedAt).default(Expr::current_timestamp()))
                .col(date_time_null(Poetry::UpdatedAt).default(Expr::current_timestamp()))
                .to_owned(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("创建 poetry 表失败: {}", e))?;
    
    println!("poetry 表创建成功");

    // 创建 poetry_lines 表
    manager
        .create_table(
            Table::create()
                .table(PoetryLines::Table)
                .if_not_exists()
                .col(pk_auto(PoetryLines::Id))
                .col(integer(PoetryLines::PoetryId).default(0))
                .col(small_integer(PoetryLines::LineNumber).default(0))
                .col(text(PoetryLines::Content).default(""))
                .col(text(PoetryLines::Pinyin).default(""))
                .col(text(PoetryLines::Description).default(""))
                .col(text(PoetryLines::Notes).default(""))
                .col(date_time(PoetryLines::CreatedAt).default(Expr::current_timestamp()))
                .col(date_time_null(PoetryLines::UpdatedAt).default(Expr::current_timestamp()))
                .to_owned(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("创建 poetry_lines 表失败: {}", e))?;

    println!("poetry_lines 表创建成功");

    // 创建 chapters 表
    manager
        .create_table(
            Table::create()
                .table(Chapters::Table)
                .if_not_exists()
                .col(pk_auto(Chapters::Id))
                .col(string_len_uniq(Chapters::Uuid, 32))
                .col(integer(Chapters::Pid).default(0))
                .col(integer(Chapters::PoetryId).default(0))
                .col(string_len(Chapters::Title, 255))
                .col(text(Chapters::Content).default(""))
                .col(small_integer(Chapters::WordCount).default(0))
                .col(small_integer(Chapters::Weight).default(0))
                .col(date_time(Chapters::CreatedAt).default(Expr::current_timestamp()))
                .col(date_time_null(Chapters::UpdatedAt).default(Expr::current_timestamp()))
                .to_owned(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("创建 chapters 表失败: {}", e))?;

    println!("chapters 表创建成功");

    // 创建 chapter_lines 表
    manager
        .create_table(
            Table::create()
                .table(ChapterLines::Table)
                .if_not_exists()
                .col(pk_auto(ChapterLines::Id))
                .col(integer(ChapterLines::ChapterId).default(0))
                .col(small_integer(ChapterLines::LineNumber).default(0))
                .col(text(ChapterLines::Content).default(""))
                .col(text(ChapterLines::Pinyin).default(""))
                .col(text(ChapterLines::Description).default(""))
                .col(text(ChapterLines::Notes).default(""))
                .col(date_time(ChapterLines::CreatedAt).default(Expr::current_timestamp()))
                .col(date_time_null(ChapterLines::UpdatedAt).default(Expr::current_timestamp()))
                .to_owned(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("创建 chapter_lines 表失败: {}", e))?;

    println!("chapter_lines 表创建成功");

    // 创建 categories 表
    manager
        .create_table(
            Table::create()
                .table(Categories::Table)
                .if_not_exists()
                .col(pk_auto(Categories::Id))
                .col(string_len_uniq(Categories::Name, 100))
                .col(text(Categories::Description).default(""))
                .to_owned(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("创建 categories 表失败: {}", e))?;

    println!("categories 表创建成功");
    Ok(())
}

pub async fn delete_tables() -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let manager = SchemaManager::new(db);
    manager
        .drop_table(Table::drop().table(Authors::Table).if_exists().to_owned())
        .await
        .map_err(|e| anyhow::anyhow!("删除 authors 表失败: {}", e))?;
    manager
        .drop_table(Table::drop().table(Poetry::Table).if_exists().to_owned())
        .await
        .map_err(|e| anyhow::anyhow!("删除 poetry 表失败: {}", e))?;
    manager
        .drop_table(Table::drop().table(PoetryLines::Table).if_exists().to_owned())
        .await
        .map_err(|e| anyhow::anyhow!("删除 poetry_lines 表失败: {}", e))?;
    manager
        .drop_table(Table::drop().table(Chapters::Table).if_exists().to_owned())
        .await
        .map_err(|e| anyhow::anyhow!("删除 chapters 表失败: {}", e))?;
    manager
        .drop_table(Table::drop().table(ChapterLines::Table).if_exists().to_owned())
        .await
        .map_err(|e| anyhow::anyhow!("删除 chapter_lines 表失败: {}", e))?;
    manager
        .drop_table(Table::drop().table(Categories::Table).if_exists().to_owned())
        .await
        .map_err(|e| anyhow::anyhow!("删除 categories 表失败: {}", e))?;
    Ok(())
}