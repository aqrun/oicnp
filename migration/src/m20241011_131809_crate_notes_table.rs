use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::sea_query::Expr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Notes {
    Table,
    Id,
    Title,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notes::Table)
                    .if_not_exists()
                    .col(pk_auto(Notes::Id))
                    .col(string_null(Notes::Title))
                    .col(string_null(Notes::Content))
                    .col(
                        ColumnDef::new(Notes::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Notes::UpdatedAt)
                            .date_time()
                            .default(Value::Int(None)),
                    )
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

