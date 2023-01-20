use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212320_create_user_online_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysUserOnline::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysUserOnline::Uid)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysUserOnline::TokenId).string_len(32).default(""))
            .col(ColumnDef::new(SysUserOnline::TokenExpire).big_integer().default("0"))
            .col(ColumnDef::new(SysUserOnline::LoginAt)
                .date_time()
                .default(Value::Int(None)))
            .col(ColumnDef::new(SysUserOnline::Username).string_len(60).default(""))
            .col(ColumnDef::new(SysUserOnline::DepartmentName).string_len(100).default(""))
            .col(ColumnDef::new(SysUserOnline::Net).string_len(10).default(""))
            .col(ColumnDef::new(SysUserOnline::Ip).string_len(100).default(""))
            .col(ColumnDef::new(SysUserOnline::Location).string_len(255).default(""))
            .col(ColumnDef::new(SysUserOnline::Device).string_len(50).default(""))
            .col(ColumnDef::new(SysUserOnline::Browser).string_len(30).default(""))
            .col(ColumnDef::new(SysUserOnline::Os).string_len(30).default(""))
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysUserOnline::Table).to_owned()
        ).await
    }
}
