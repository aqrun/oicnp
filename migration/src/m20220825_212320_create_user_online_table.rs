use sea_orm_migration::prelude::*;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(UserOnline::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserOnline::Uid)
                    .big_integer()
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(UserOnline::TokenId).string_len(32).not_null().default(""))
            .col(ColumnDef::new(UserOnline::TokenExpire).big_integer().not_null().default("0"))
            .col(ColumnDef::new(UserOnline::LoginAt)
                .date_time()
                .not_null()
                .default(Value::Int(None)))
            .col(ColumnDef::new(UserOnline::Username).string_len(60).not_null().default(""))
            .col(ColumnDef::new(UserOnline::DptName).string_len(100).not_null().default(""))
            .col(ColumnDef::new(UserOnline::Net).string_len(10).not_null().default(""))
            .col(ColumnDef::new(UserOnline::Ip).string_len(100).not_null().default(""))
            .col(ColumnDef::new(UserOnline::Location).string_len(255).not_null().default(""))
            .col(ColumnDef::new(UserOnline::Device).string_len(50).not_null().default(""))
            .col(ColumnDef::new(UserOnline::Browser).string_len(30).not_null().default(""))
            .col(ColumnDef::new(UserOnline::Os).string_len(30).not_null().default(""))
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(UserOnline::Table).to_owned()
        ).await
    }
}
