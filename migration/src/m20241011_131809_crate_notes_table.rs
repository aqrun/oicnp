use sea_orm_migration::{prelude::*, schema::*};
use loco_rs::schema::table_auto;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Notes {
    Table,
    Id,
    Title,
    Content,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Notes::Table)
                    .col(pk_auto(Notes::Id))
                    .col(string_null(Notes::Title))
                    .col(string_null(Notes::Content))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notes::Table).to_owned())
            .await
    }
}

