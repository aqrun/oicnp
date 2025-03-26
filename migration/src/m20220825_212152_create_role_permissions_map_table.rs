use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(RolePermissionsMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(RolePermissionsMap::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(RolePermissionsMap::RoleId).big_integer().not_null().default(0))
            .col(ColumnDef::new(RolePermissionsMap::PermissionId).string_len(255).not_null().default(""))
            .col(ColumnDef::new(RolePermissionsMap::Method).string_len(10).not_null().default(0))
            .col(ColumnDef::new(RolePermissionsMap::CreatedBy).big_integer().not_null().default(0))
            .col(
                ColumnDef::new(RolePermissionsMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(RolePermissionsMap::Table).to_owned()
        ).await
    }
}
