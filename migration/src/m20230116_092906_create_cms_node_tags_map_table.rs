use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(NodeTagsMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(NodeTagsMap::Bundle).string_len(32).not_null().default(""))
            .col(ColumnDef::new(NodeTagsMap::Nid).big_integer().not_null().default(0))
            .col(ColumnDef::new(NodeTagsMap::TagId).big_integer().not_null().default(0))
            .primary_key(
                Index::create()
                    .col(NodeTagsMap::Nid)
                    .col(NodeTagsMap::TagId),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NodeTagsMap::Table).to_owned())
            .await
    }
}
