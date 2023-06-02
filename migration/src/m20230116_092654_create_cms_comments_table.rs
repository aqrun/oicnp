use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092654_create_cms_comments_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsComments::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CmsComments::Cid)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CmsComments::Uid).string_len(32).default(""))
            .col(ColumnDef::new(CmsComments::Pid).string_len(32).default(""))
            .col(ColumnDef::new(CmsComments::Status).char_len(1).default("0"))
            .col(ColumnDef::new(CmsComments::Bundle).string_len(64).default(""))
            .col(ColumnDef::new(CmsComments::TargetId).string_len(32).default(""))
            .col(ColumnDef::new(CmsComments::Subject).string_len(512).default(""))
            .col(ColumnDef::new(CmsComments::Name).string_len(128).default(""))
            .col(ColumnDef::new(CmsComments::Email).string_len(128).default(""))
            .col(ColumnDef::new(CmsComments::Homepage).string_len(128).default(""))
            .col(ColumnDef::new(CmsComments::Hostname).string_len(128).default(""))
            .col(ColumnDef::new(CmsComments::CreatedBy).string_len(32).default(""))
            .col(ColumnDef::new(CmsComments::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(CmsComments::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(CmsComments::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None))
            )
            .col(
                ColumnDef::new(CmsComments::DeletedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsComments::Table).to_owned())
            .await
    }
}
