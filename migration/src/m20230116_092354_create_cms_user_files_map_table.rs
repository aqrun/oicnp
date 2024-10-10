use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(UserFilesMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserFilesMap::Uid)
                    .big_integer()
                    .not_null()
                    .default(0),
            )
            .col(
                ColumnDef::new(UserFilesMap::FileId)
                .big_integer()
                .not_null()
                .default(0),
            )
            .col(ColumnDef::new(UserFilesMap::Bundle).string_len(32).not_null().default(""))
            .col(ColumnDef::new(UserFilesMap::Weight).integer().not_null().default(0))
            .col(ColumnDef::new(UserFilesMap::Alt).string_len(512).not_null().default(""))
            .col(ColumnDef::new(UserFilesMap::Title).string_len(512).not_null().default(""))
            .col(ColumnDef::new(UserFilesMap::Width).big_integer().not_null().default(0))
            .col(ColumnDef::new(UserFilesMap::Height).big_integer().not_null().default(0))
            .primary_key(
                Index::create()
                    .col(UserFilesMap::Uid)
                    .col(UserFilesMap::FileId),
            )
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserFilesMap::Table).to_owned())
            .await
    }
}
