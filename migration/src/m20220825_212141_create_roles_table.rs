use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220825_212141_create_roles_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(SysRoles::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SysRoles::Id)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(SysRoles::Vid).string_len(64).default(""))
            .col(ColumnDef::new(SysRoles::Name).string_len(64).default(""))
            .col(ColumnDef::new(SysRoles::Weight).integer().default(0))
            .col(ColumnDef::new(SysRoles::Scope).char_len(1).default("0"))
            .col(ColumnDef::new(SysRoles::Status).char_len(1).default(""))
            .col(ColumnDef::new(SysRoles::Remark).string_len(255).default(""))
            .col(
                ColumnDef::new(SysRoles::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysRoles::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop_table(SysRoles::Table).to_owned()
        ).await
    }
}
