use oicnp_core::prelude::sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092826_create_cms_node_taxonomies_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsNodeTaxonomiesMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(CmsNodeTaxonomiesMap::Bundle).string_len(20).default(""))
            .col(ColumnDef::new(CmsNodeTaxonomiesMap::Nid).string_len(32).default(""))
            .col(ColumnDef::new(CmsNodeTaxonomiesMap::Tid).string_len(32).default(""))
            .primary_key(
                Index::create()
                    .col(CmsNodeTaxonomiesMap::Nid)
                    .col(CmsNodeTaxonomiesMap::Tid),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsNodeTaxonomiesMap::Table).to_owned())
            .await
    }
}
