use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092740_create_cms_nodes_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsNodes::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CmsNodes::Nid)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CmsNodes::Vid).string_len(255).default(""))
            .col(ColumnDef::new(CmsNodes::Bundle).string_len(64).default(""))
            .col(ColumnDef::new(CmsNodes::Title).string_len(512).default(""))
            .col(ColumnDef::new(CmsNodes::Viewed).integer().default(0))
            .col(ColumnDef::new(CmsNodes::Deleted).char_len(1).default("0"))
            .col(
                ColumnDef::new(CmsNodes::PublishedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(ColumnDef::new(CmsNodes::CreatedBy).string_len(32).default(""))
            .col(ColumnDef::new(CmsNodes::UpdatedBy).string_len(32).default(""))
            .col(
                ColumnDef::new(CmsNodes::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
            )
            .col(
                ColumnDef::new(CmsNodes::UpdatedAt)
                    .date_time()
                    .default(Value::Int(None)),
            )
            .col(
                ColumnDef::new(CmsNodes::DeletedAt)
                    .date_time()
                    .default(Value::Int(None))
            )
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsNodes::Table).to_owned())
            .await
    }
}
