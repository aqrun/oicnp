use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212250_create_user_department_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysUserDepartmentMap::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysUserDepartmentMap::Uid)
                    .string_len(32)
                    .not_null(),
            )
            .col(
                ColumnDef::new(SysUserDepartmentMap::DepartmentId)
                    .string_len(32)
                    .not_null(),
            )
            .col(ColumnDef::new(SysUserDepartmentMap::CreatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysUserDepartmentMap::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .primary_key(
                Index::create()
                    .col(SysUserDepartmentMap::Uid)
                    .col(SysUserDepartmentMap::DepartmentId),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysUserDepartmentMap::Table).to_owned()
        ).await
    }
}
