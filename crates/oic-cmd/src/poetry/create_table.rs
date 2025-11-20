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
    BirthAt,
    DeathAt,
    Dynasty,
    Weight,
    CreatedAt,
    UpdatedAt,
}

pub async fn create_tables() -> Result<()> {
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
                .col(date_time(Authors::BirthAt))
                .col(date_time(Authors::DeathAt))
                .col(string_len(Authors::Dynasty, 20).default(""))
                .col(small_integer(Authors::Weight).default(0))
                .col(date_time(Authors::CreatedAt).default(Expr::current_timestamp()))
                .col(date_time(Authors::UpdatedAt).default(Expr::current_timestamp()))
                .to_owned(),
        )
        .await
        .map_err(|e| anyhow::anyhow!("创建 authors 表失败: {}", e))?;

    println!("authors 表创建成功");
    Ok(())
}