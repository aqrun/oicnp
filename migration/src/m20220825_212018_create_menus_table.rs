use sea_orm_migration::{prelude::*, schema::*};
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Menus::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Menus::Id)
                    .integer()
                    .not_null()
                    .primary_key()
                    .auto_increment(),
            )
            .col(ColumnDef::new(Menus::Vid).unique_key().string_len(255).not_null())
            .col(ColumnDef::new(Menus::Pid).string_len(255).not_null())
            .col(ColumnDef::new(Menus::Path).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Menus::Name).string_len(100).not_null().default(""))
            .col(integer(Menus::Depth).default(0))
            .col(integer(Menus::P1).default(0))
            .col(integer(Menus::P2).default(0))
            .col(integer(Menus::P3).default(0))
            .col(integer(Menus::P4).default(0))
            .col(integer(Menus::P5).default(0))
            .col(integer(Menus::P6).default(0))
            .col(integer(Menus::P7).default(0))
            .col(integer(Menus::P8).default(0))
            .col(ColumnDef::new(Menus::Icon).string_len(50).not_null().default(""))
            .col(ColumnDef::new(Menus::Weight).integer().not_null().default(0))
            .col(ColumnDef::new(Menus::Api).string_len(255).not_null().default(""))
            .col(ColumnDef::new(Menus::Status).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(Menus::Visible).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(Menus::IsCache).char_len(1).not_null().default("1"))
            .col(ColumnDef::new(Menus::IsFrame).char_len(1).not_null().default("0"))
            .col(ColumnDef::new(Menus::Remark).string_len(255).not_null().default(""))
            .col(
                ColumnDef::new(Menus::CreatedAt)
                    .date_time()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(Menus::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(Menus::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(Menus::Table).to_owned()
        ).await
    }
}

