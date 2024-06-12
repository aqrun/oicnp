use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Categories::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Categories::CatId)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Categories::CatVid).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Categories::CatPid).big_integer().not_null().default(0))
            .col(ColumnDef::new(Categories::CatName).string_len(128).not_null().default(""))
            .col(ColumnDef::new(Categories::CatDesc).string_len(512).not_null().default(""))
            .col(ColumnDef::new(Categories::CatDescFormat).string_len(20).not_null().default(""))
            .col(ColumnDef::new(Categories::Weight).integer().not_null().default(0))
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}
