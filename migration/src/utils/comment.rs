use oicnp_core::prelude::sea_orm_migration::prelude::*;

pub async fn comment_table(
    db: &DatabaseConnection,
    table_name: &str,
    comment: &str,
) -> Result<(), DbErr> {
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("COMMENT ON TABLE \"{}\" IS \"{}\";", table_name, comment),
    ))
    .await?
}

pub async fn comment_column(
    db: &DatabaseConnection,
    table_name: &str,
    column_name: &str,
    comment: &str,
) -> Result<(), DbErr> {
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("COMMENT ON COLUMN \"{}\".{} IS \"{}\";", table_name, column_name, comment),
    ))
    .await?
}
