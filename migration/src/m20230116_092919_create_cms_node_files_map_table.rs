use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(NodeFilesMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(NodeFilesMap::Bundle).string_len(20).not_null().default(""))
            .col(ColumnDef::new(NodeFilesMap::Nid).big_integer().not_null().default(0))
            .col(ColumnDef::new(NodeFilesMap::FileId).big_integer().not_null().default(0))
            .col(ColumnDef::new(NodeFilesMap::Usage).string_len(64).not_null().default(""))
            .col(ColumnDef::new(NodeFilesMap::Weight).integer().not_null().default(0))
            .col(ColumnDef::new(NodeFilesMap::Alt).string_len(512).not_null().default(""))
            .col(ColumnDef::new(NodeFilesMap::Title).string_len(512).not_null().default(""))
            .col(ColumnDef::new(NodeFilesMap::Width).big_integer().not_null().default(0))
            .col(ColumnDef::new(NodeFilesMap::Height).big_integer().not_null().default(0))
            .primary_key(
                Index::create()
                    .col(NodeFilesMap::Nid)
                    .col(NodeFilesMap::FileId),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NodeFilesMap::Table).to_owned())
            .await
    }
}
