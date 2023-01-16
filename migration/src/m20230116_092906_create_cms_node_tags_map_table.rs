use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092906_create_cms_node_tags_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsNodeTagsMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(CmsNodeTagsMap::Bundle).string_len(20).default(""))
            .col(ColumnDef::new(CmsNodeTagsMap::Nid).string_len(32).default(""))
            .col(ColumnDef::new(CmsNodeTagsMap::TagId).string_len(32).default(""))
            .primary_key(
                Index::create()
                    .col(CmsNodeTagsMap::Nid)
                    .col(CmsNodeTagsMap::TagId),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsNodeTagsMap::Table).to_owned())
            .await
    }
}
