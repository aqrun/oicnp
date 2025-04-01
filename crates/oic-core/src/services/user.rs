use loco_rs::prelude::*;
use sea_orm::{
    prelude::*,
    // DbBackend,
    // QueryTrait,
    FromQueryResult,
    QuerySelect,
};
use sea_orm::query::Condition;
use crate::entities::{prelude::*, role};

#[derive(Debug, FromQueryResult)]
pub struct UserRoleRes {
    pub uid: i64,
    pub role_id: i64,
    pub vid: String,
    pub name: String,
}

///
/// 检测用户是否具有管理员角色
/// 
/// SELECT user_role_map.uid,roles.role_id,roles.vid
/// FROM public.roles
/// left join user_role_map
/// on roles.role_id = user_role_map.role_id
/// where user_role_map.uid = 1
/// 
pub async fn check_user_is_admin(db: &DatabaseConnection, uid: i64) -> Result<bool> {
    let user = UserModel::find_by_uid(db, uid).await?;

    if user.is_admin.eq("1") {
        return Ok(true);
    }

    let q = RoleEntity::find()
        .select_only()
        .column(UserRoleMapColumn::Uid)
        .column(RoleColumn::RoleId)
        .column(RoleColumn::Vid)
        .column(RoleColumn::Name)
        .join(
            sea_orm::JoinType::LeftJoin, 
            role::Relation::UserRole.def()
        )
        .filter(
            Condition::all()
                .add(RoleColumn::Vid.eq("admin"))
                .add(UserRoleMapColumn::Uid.eq(uid))
        );

    /*
    let sql = q.clone()
        .build(DbBackend::Postgres)
        .to_string();
    println!("rols---{:?}", sql);
    */
    let role_res = q
        .into_model::<UserRoleRes>()
        .one(db)
        .await?;
    
    // println!("rol ----- {:?}", roles);
    Ok(role_res.is_some())
}

///
/// 检测用户是否存在指定角色
/// 
pub async fn check_user_has_role(db: &DatabaseConnection, uid: i64, role_vid: &str) -> Result<bool> {
    let q = RoleEntity::find()
        .select_only()
        .column(UserRoleMapColumn::Uid)
        .column(RoleColumn::RoleId)
        .column(RoleColumn::Vid)
        .column(RoleColumn::Name)
        .join(
            sea_orm::JoinType::LeftJoin, 
            role::Relation::UserRole.def()
        )
        .filter(
            Condition::all()
                .add(RoleColumn::Vid.eq(role_vid))
                .add(UserRoleMapColumn::Uid.eq(uid))
        );
    /*
    let sql = q.clone()
        .build(DbBackend::Postgres)
        .to_string();
    println!("rols---{:?}", sql);
    */
    let role_res = q
        .into_model::<UserRoleRes>()
        .one(db)
        .await?;
    
    // println!("rol ----- {:?}", role_res);
    Ok(role_res.is_some())
}

///
/// 检测用户是否具体指定权限
///
pub async fn can(_db: &DatabaseConnection, _uid: i64, _permission_vid: &str) -> Result<bool> {
    Ok(true)
}
