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
                    .col(
                        ColumnDef::new(Notes::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(string(Notes::Title))
                    .col(text(Notes::Content).default(""))
                    .col(
                        date_time(Notes::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time_null(Notes::UpdatedAt)
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

