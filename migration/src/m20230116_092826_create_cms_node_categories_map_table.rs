use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(NodeCategoriesMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(NodeCategoriesMap::Bundle).string_len(20).not_null().default(""))
            .col(ColumnDef::new(NodeCategoriesMap::Nid).big_integer().not_null().default(0))
            .col(ColumnDef::new(NodeCategoriesMap::CatId).big_integer().not_null().default(0))
            .primary_key(
                Index::create()
                    .col(NodeCategoriesMap::Nid)
                    .col(NodeCategoriesMap::CatId),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NodeCategoriesMap::Table).to_owned())
            .await
    }
}
