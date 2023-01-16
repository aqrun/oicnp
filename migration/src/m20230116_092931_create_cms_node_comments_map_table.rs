use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092931_create_cms_node_comments_map_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsNodeCommentsMap::Table)
            .if_not_exists()
            .col(ColumnDef::new(CmsNodeCommentsMap::Bundle).string_len(20).default(""))
            .col(ColumnDef::new(CmsNodeCommentsMap::Nid).string_len(32).default(""))
            .col(ColumnDef::new(CmsNodeCommentsMap::CommentId).string_l(32).default(""))
            .primary_key(
                Index::create()
                    .col(CmsNodeCommentsMap::Nid)
                    .col(CmsNodeCommentsMap::CommentId),
            )
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsNodeCommentsMap::Table).to_owned())
            .await
    }
}
