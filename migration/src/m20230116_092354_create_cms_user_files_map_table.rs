use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092354_create_cms_user_files_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsUserFilesMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(CmsUserFilesMap::Uid).string_len(32).default(""))
            .col(ColumnDef::new(CmsUserFilesMap::Fid).string_len(32).default(""))
            .col(ColumnDef::new(CmsUserFilesMap::Bundle).string_len(32).default(""))
            .col(ColumnDef::new(CmsUserFilesMap::Weight).integer().default(0))
            .col(ColumnDef::new(CmsUserFilesMap::Alt).string_len(512).default(""))
            .col(ColumnDef::new(CmsUserFilesMap::Title).string_len(512).default(""))
            .col(ColumnDef::new(CmsUserFilesMap::Width).big_integer().default(0))
            .col(ColumnDef::new(CmsUserFilesMap::Height).big_integer().default(0))
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsUserFilesMap::Table).to_owned())
            .await
    }
}
