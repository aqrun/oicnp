use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(OperationLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(OperationLogs::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(OperationLogs::TimeId).big_integer().not_null().default(0))
            .col(ColumnDef::new(OperationLogs::Title).string_len(50).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::BusinessType).string_len(100).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::Method).string_len(100).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::RequestMethod).string_len(100).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::OperatorType).string_len(100).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::Name).string_len(50).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::DepartmentName).string_len(50).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::Url).string_len(255).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::Ip).string_len(50).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::Location).string_len(255).not_null().default(""))
            .col(ColumnDef::new(OperationLogs::Param).text().not_null().default(""))
            .col(ColumnDef::new(OperationLogs::PathParam).text().not_null().default(""))
            .col(ColumnDef::new(OperationLogs::JsonResult).text().not_null().default(""))
            .col(ColumnDef::new(OperationLogs::Status).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(OperationLogs::ErrorMessage).text().not_null().default(""))
            .col(ColumnDef::new(OperationLogs::Duration).big_integer().not_null().default(1))
            .col(
                ColumnDef::new(OperationLogs::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(OperationLogs::Table).to_owned()
        ).await
    }
}
