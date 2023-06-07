use oicnp_core::{
    DB,DatabaseConnection, establish_connection,
    prelude::{
        anyhow::{Result},
        sea_orm_migration::prelude::*,
    }
};
use migration::types as tables;

pub async fn truncate_tables() {
    let db = DB.get_or_init(establish_connection).await;
    truncate_all_tables(db).await;
}

pub async fn truncate_all_tables(db: &DatabaseConnection) {
    let manager = SchemaManager::new(db);

    manager.truncate_table(Table::truncate().table(tables::CmsNodes::Table).to_owned())
        .await.expect("truncate failed");
    manager.truncate_table(Table::truncate().table(tables::CmsTaxonomies::Table).to_owned())
        .await.expect("truncate failed");
    manager.truncate_table(Table::truncate().table(tables::CmsNodeBody::Table).to_owned())
        .await.expect("truncate failed");
    manager.truncate_table(Table::truncate().table(tables::CmsNodeTaxonomiesMap::Table).to_owned())
        .await.expect("truncate failed");
    println!("truncat table complete!!!");
}