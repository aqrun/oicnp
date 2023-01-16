use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212215_create_role_department_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysRoleDepartmentMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysRoleDepartmentMap::RoleId)
                    .string_len(32)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysRoleDepartmentMap::DepartmentId)
                    .string_len(32)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysRoleDepartmentMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .primary_key(
                Index::create()
                    .col(SysRoleDepartmentMap::RoleId)
                    .col(SysRoleDepartmentMap::DepartmentId)
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysRoleDepartmentMap::Table).to_owned()
        ).await
    }
}
