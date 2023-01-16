use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092919_create_cms_node_files_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsNodeFilesMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(CmsNodeFilesMap::Bundle).string_len(20).default(""))
            .col(ColumnDef::new(CmsNodeFilesMap::Nid).string_len(32).default(""))
            .col(ColumnDef::new(CmsNodeFilesMap::Fid).string_len(32).default(""))
            .col(ColumnDef::new(CmsNodeFilesMap::Usage).string_len(64).default(""))
            .col(ColumnDef::new(CmsNodeFilesMap::Weight).integer().default(0))
            .col(ColumnDef::new(CmsNodeFilesMap::Alt).string_len(512).default(""))
            .col(ColumnDef::new(CmsNodeFilesMap::Title).string_len(512).default(""))
            .col(ColumnDef::new(CmsNodeFilesMap::Width).big_integer().default(0))
            .col(ColumnDef::new(CmsNodeFilesMap::Height).big_integer().default(0))
            .primary_key(
                Index::create()
                    .col(CmsNodeFilesMap::Nid)
                    .col(CmsNodeFilesMap::Fid),
            )
            .to_owned();

        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsNodeFilesMap::Table).to_owned())
            .await
    }
}
