use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_090508_create_cms_files_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let cms_files_table = Table::create()
            .table(CmsFiles::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CmsFiles::Fid)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CmsFiles::Uid).string_len(32).default(""))
            .col(ColumnDef::new(CmsFiles::Filename).string_len(255).default(""))
            .col(ColumnDef::new(CmsFiles::Uri).string_len(255).default(""))
            .col(ColumnDef::new(CmsFiles::Storage).string_len(64).default(""))
            .col(ColumnDef::new(CmsFiles::Mime).string_len(64).default(""))
            .col(ColumnDef::new(CmsFiles::Status).char_len(1).default("0"))
            .col(ColumnDef::new(SysUsers::CreatedBy).string_len(32).not_null())
            .col(ColumnDef::new(SysUsers::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(SysUsers::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(SysUsers::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(SysUsers::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(cms_files_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            Table::drop_table(CmsFiles::Table).to_owned()
        ).await
    }
}

