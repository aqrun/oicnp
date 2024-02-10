use oicnp_core::{
    DB,DatabaseConnection, establish_connection,
};
use migration::types as tables;
use sea_orm_migration::prelude::*;

pub async fn truncate_tables() {
    let db = DB.get_or_init(establish_connection).await;
    truncate_all_tables(db).await;
}

pub async fn truncate_all_tables(db: &DatabaseConnection) {
    let manager = SchemaManager::new(db);

    manager.truncate_table(Table::truncate().table(tables::Nodes::Table).to_owned())
        .await.expect("truncate failed");
    manager.truncate_table(Table::truncate().table(tables::Categories::Table).to_owned())
        .await.expect("truncate failed");
    manager.truncate_table(Table::truncate().table(tables::Tags::Table).to_owned())
        .await.expect("truncate failed");
    manager.truncate_table(Table::truncate().table(tables::NodeBody::Table).to_owned())
        .await.expect("truncate failed");
    manager.truncate_table(Table::truncate().table(tables::NodeCategoriesMap::Table).to_owned())
        .await.expect("truncate failed");
    manager.truncate_table(Table::truncate().table(tables::NodeTagsMap::Table).to_owned())
        .await.expect("truncate failed");
    println!("数据表清理完成！");
}