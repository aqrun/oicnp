use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Configs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Configs::Name)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Configs::Data).string_len(512).not_null().default(""))
            .col(ColumnDef::new(Configs::DataType).string_len(32).not_null().default(""))
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Configs::Table).to_owned())
            .await
    }
}
