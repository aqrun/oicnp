use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092759_create_cms_node_body_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsNodeBody::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(CmsNodeBody::Nid)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key()
            )
            .col(ColumnDef::new(CmsNodeBody::Summary).text().default(""))
            .col(ColumnDef::new(CmsNodeBody::SummaryFormat).string_len(20).default(""))
            .col(ColumnDef::new(CmsNodeBody::Body).text().default(""))
            .col(ColumnDef::new(CmsNodeBody::BodyFormat).string_len(20).default(""))
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsNodeBody::Table).to_owned())
            .await
    }
}
