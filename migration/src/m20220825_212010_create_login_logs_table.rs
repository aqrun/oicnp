use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212010_create_login_logs_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysLoginLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysLoginLogs::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysLoginLogs::LoginName).string_len(50).default(""))
            .col(ColumnDef::new(SysLoginLogs::Net).string_len(10).default(""))
            .col(ColumnDef::new(SysLoginLogs::Ip).string_len(50).default(""))
            .col(ColumnDef::new(SysLoginLogs::Location).string_len(255).default(""))
            .col(ColumnDef::new(SysLoginLogs::Browser).string_len(50).default(""))
            .col(ColumnDef::new(SysLoginLogs::Os).string_len(50).default(""))
            .col(ColumnDef::new(SysLoginLogs::Device).string_len(50).default(""))
            .col(ColumnDef::new(SysLoginLogs::Status).char_len(1).default("1"))
            .col(ColumnDef::new(SysLoginLogs::Message).string_len(255).default(""))
            .col(ColumnDef::new(SysLoginLogs::Module).string_len(30).default(""))
            .col(
                ColumnDef::new(SysLoginLogs::LoginAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysLoginLogs::Table).to_owned()
        ).await
    }
}

