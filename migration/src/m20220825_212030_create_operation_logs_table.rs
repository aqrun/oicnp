use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212030_create_operation_logs_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysMenus::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysOperationLogs::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysOperationLogs::TimeId).big_integer().not_null().default(0))
            .col(ColumnDef::new(SysOperationLogs::Title).string_len(50).default(""))
            .col(ColumnDef::new(SysOperationLogs::BusinessType).string_len(100).default(""))
            .col(ColumnDef::new(SysOperationLogs::Method).string_len(100).default(""))
            .col(ColumnDef::new(SysOperationLogs::RequestMethod).string_len(100).default(""))
            .col(ColumnDef::new(SysOperationLogs::OperatorType).string_len(100).default(""))
            .col(ColumnDef::new(SysOperationLogs::Name).string_len(50).default(""))
            .col(ColumnDef::new(SysOperationLogs::DepartmentName).string_len(50).default(""))
            .col(ColumnDef::new(SysOperationLogs::Url).string_len(255).default(""))
            .col(ColumnDef::new(SysOperationLogs::Ip).string_len(50).default(""))
            .col(ColumnDef::new(SysOperationLogs::Location).string_len(255).default(""))
            .col(ColumnDef::new(SysOperationLogs::Param).text().default(""))
            .col(ColumnDef::new(SysOperationLogs::PathParam).text().default(""))
            .col(ColumnDef::new(SysOperationLogs::JsonResult).text().default(""))
            .col(ColumnDef::new(SysOperationLogs::Status).char_len(1).default("1"))
            .col(ColumnDef::new(SysOperationLogs::ErrorMessage).text().default(""))
            .col(ColumnDef::new(SysOperationLogs::Duration).big_integer().default(1))
            .col(
                ColumnDef::new(SysOperationLogs::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysOperationLogs::Table).to_owned()
        ).await
    }
}
