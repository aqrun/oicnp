use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212018_create_menus_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysMenus::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysMenus::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysMenus::Pid).string_len(32).not_null())
            .col(ColumnDef::new(SysMenus::Path).string_len(255).default(""))
            .col(ColumnDef::new(SysMenus::Name).string_len(100).default(""))
            .col(ColumnDef::new(SysMenus::Icon).string_len(50).default(""))
            .col(ColumnDef::new(SysMenus::Type).char_len(1).default(""))
            .col(ColumnDef::new(SysMenus::Query).string_len(255).default(""))
            .col(ColumnDef::new(SysMenus::Weight).integer().default(0))
            .col(ColumnDef::new(SysMenus::Api).string_len(255).default(""))
            .col(ColumnDef::new(SysMenus::Status).char_len(1).default("1"))
            .col(ColumnDef::new(SysMenus::Method).string_len(10).default(""))
            .col(ColumnDef::new(SysMenus::Component).string_len(100).default(""))
            .col(ColumnDef::new(SysMenus::Visible).char_len(1).default("1"))
            .col(ColumnDef::new(SysMenus::IsCache).char_len(1).default("1"))
            .col(ColumnDef::new(SysMenus::LogMethod).char_len(1).default("0"))
            .col(ColumnDef::new(SysMenus::DataCacheMethod).char_len(1).default("0"))
            .col(ColumnDef::new(SysMenus::IsFrame).char_len(1).default("0"))
            .col(ColumnDef::new(SysMenus::DataScope).char_len(1).default("0"))
            .col(ColumnDef::new(SysMenus::Remark).string_len(255).default(""))
            .col(
                ColumnDef::new(SysMenus::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysMenus::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysMenus::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop().table(SysMenus::Table).to_owned()
        ).await
    }
}

