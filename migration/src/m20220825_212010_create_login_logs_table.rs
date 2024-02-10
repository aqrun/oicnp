use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(LoginLogs::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(LoginLogs::Id)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(LoginLogs::LoginName).string_len(50).not_null().default(""))
            .col(ColumnDef::new(LoginLogs::Net).string_len(10).not_null().default(""))
            .col(ColumnDef::new(LoginLogs::Ip).string_len(50).not_null().default(""))
            .col(ColumnDef::new(LoginLogs::Location).string_len(255).not_null().default(""))
            .col(ColumnDef::new(LoginLogs::Browser).string_len(50).not_null().default(""))
            .col(ColumnDef::new(LoginLogs::Os).string_len(50).not_null().default(""))
            .col(ColumnDef::new(LoginLogs::Device).string_len(50).not_null().default(""))
            .col(ColumnDef::new(LoginLogs::Status).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(LoginLogs::Message).string_len(255).not_null().default(""))
            .col(ColumnDef::new(LoginLogs::Module).string_len(30).not_null().default(""))
            .col(
                ColumnDef::new(LoginLogs::LoginAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(LoginLogs::Table).to_owned()
        ).await
    }
}

