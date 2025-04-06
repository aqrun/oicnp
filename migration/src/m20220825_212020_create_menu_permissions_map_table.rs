use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(MenuPermissionsMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(MenuPermissionsMap::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(MenuPermissionsMap::MenuId).big_integer().not_null().default(0))
            .col(ColumnDef::new(MenuPermissionsMap::PermissionId).big_integer().not_null().default(0))
            .col(ColumnDef::new(MenuPermissionsMap::CreatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(MenuPermissionsMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(MenuPermissionsMap::Table).to_owned()
        ).await
    }
}
