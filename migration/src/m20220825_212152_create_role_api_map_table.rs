use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212152_create_role_api_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysRoleApiMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysRoleApiMap::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysRoleApiMap::RoleId).string_len(32).default(""))
            .col(ColumnDef::new(SysRoleApiMap::Api).string_len(255).default(""))
            .col(ColumnDef::new(SysRoleApiMap::Method).string_len(10).default(0))
            .col(ColumnDef::new(SysRoleApiMap::CreatedBy).string_len(32).not_null())
            .col(
                ColumnDef::new(SysRoleApiMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysRoleApiMap::Table).to_owned()
        ).await
    }
}
