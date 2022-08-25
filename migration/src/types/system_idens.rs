use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum SysApiDb {
    Table,
    ApiId,
    Db,
}

#[derive(Iden)]
pub enum SysDepartments {
    Table,
    Id,
    ParentId,
    Name,
    Weight,
    Leader,
    Phone,
    Email,
    Status,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

pub enum SysDict