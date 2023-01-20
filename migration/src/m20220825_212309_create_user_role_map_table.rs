use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212309_create_user_role_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysUserRoleMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysUserRoleMap::Uid)
                    .string_len(32)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysUserRoleMap::RoleId)
                    .string_len(32)
                    .not_null(),
            )
            .col(ColumnDef::new(SysUserRoleMap::CreatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysUserRoleMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .primary_key(
                Index::create()
                    .col(SysUserRoleMap::Uid)
                    .col(SysUserRoleMap::RoleId)
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysUserRoleMap::Table).to_owned()
        ).await
    }
}
