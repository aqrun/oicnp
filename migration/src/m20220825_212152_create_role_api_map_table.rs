use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(RoleApiMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(RoleApiMap::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(RoleApiMap::RoleId).big_integer().not_null().default(0))
            .col(ColumnDef::new(RoleApiMap::Api).string_len(255).not_null().default(""))
            .col(ColumnDef::new(RoleApiMap::Method).string_len(10).not_null().default(0))
            .col(ColumnDef::new(RoleApiMap::CreatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(RoleApiMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(RoleApiMap::Table).to_owned()
        ).await
    }
}
