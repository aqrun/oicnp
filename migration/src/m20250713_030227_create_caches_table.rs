use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::sea_query::Expr;
use super::types::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Caches::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Caches::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Caches::CacheKey).string_len(64).not_null())
                    .col(ColumnDef::new(Caches::CacheValue).text().not_null())
                    .col(ColumnDef::new(Caches::Scope).string_len(32).not_null())
                    .col(
                        date_time(Caches::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time_null(Caches::ExpiredAt)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Caches::Table).to_owned())
            .await
    }
}

