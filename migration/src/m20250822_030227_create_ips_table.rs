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
                    .table(Ips::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ips::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Ips::Ip).string_len(64).not_null().unique_key())
                    .col(ColumnDef::new(Ips::Province).string_len(255).not_null().default(""))
                    .col(ColumnDef::new(Ips::City).string_len(255).not_null().default(""))
                    .col(ColumnDef::new(Ips::ProvinceCode).string_len(255).not_null().default(""))
                    .col(ColumnDef::new(Ips::CityCode).string_len(255).not_null().default(""))
                    .col(ColumnDef::new(Ips::Region).string_len(255).not_null().default(""))
                    .col(ColumnDef::new(Ips::RegionCode).string_len(255).not_null().default(""))
                    .col(ColumnDef::new(Ips::RegionNames).string_len(255).not_null().default(""))
                    .col(ColumnDef::new(Ips::Network).string_len(255).not_null().default(""))
                    .col(
                        date_time(Ips::CreatedAt)
                            .default(Expr::current_timestamp()),
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

