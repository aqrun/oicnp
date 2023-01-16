use sea_orm_migration::prelude::*;
use super::types::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230116_092714_create_cms_comment_body_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(CmsCommentBody::Table)
            .if_not_exist()
            .col(
                ColumnDef::new(CmsCommentBody::CommentId)
                    .string_len(32)
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(ColumnDef::new(CmsCommentBody::Body).text().default(""))
            .col(ColumnDef::new(CmsCommentBody::BodyFormat).string_len(20).default(""))
            .to_owned();
        
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CmsCommentBody::Table).to_owned())
            .await
    }
}
