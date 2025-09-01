use loco_rs::prelude::*;
use sea_orm::{
    prelude::*,
    // DbBackend,
    // QueryTrait,
    FromQueryResult,
    QuerySelect,
};
use chrono::prelude::*;
use std::time::Duration;
use sea_orm::query::Condition;
use crate::{
    entities::{prelude::*, role},
    models::{
        ModelCrudHandler,
        login_logs::CreateLoginLogReqParams,
    },
    middleware::ClientInfo,
    utils::utc_now,
    constants::{LOGIN_EXPIRE_TIME, LOGIN_REMEMBER_EXPIRE_TIME, DATE_TIME_FORMAT},
};
use super::auth::LoginResponse;

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

pub async fn add_user_login_log(
    db: &DatabaseConnection,
    info: ClientInfo,
    email: String,
    status: bool,
    message: String,
    module: String,
) -> Result<()> {
    let status_str = if status {
        "1"
    } else {
        "0"
    };

    let _ = LoginLogModel::create(db, &CreateLoginLogReqParams {
        login_name: Some(email),
        net: Some(String::from(info.network.as_str())),
        ip: Some(String::from(info.ip.as_str())),
        location: Some(String::from(info.location.as_str())),
        browser: Some(String::from(info.browser.as_str())),
        os: Some(String::from(info.os.as_str())),
        device: Some(String::from(info.device.as_str())),
        status: Some(String::from(status_str)),
        message: Some(message),
        module: Some(module),
        login_at: Some(utc_now().format(DATE_TIME_FORMAT).to_string()),
        ..Default::default()
    }).await?;

    Ok(())
}

/**
 * 更新用户在线信息
 */
pub async fn update_user_online_log(
    db: &DatabaseConnection,
    info: ClientInfo,
    login_response: LoginResponse,
) -> Result<()> {
    let user = UserModel::find_by_uid(db, login_response.uid).await?;

    let dpt = match DepartmentEntity::find()
        .filter(DepartmentColumn::Id.eq(user.dpt_id))
        .one(db)
        .await?
    {
        Some(dpt) => dpt,
        None => DepartmentModel::default(),
    };

    let token_expire = if login_response.remember {
        Utc::now() + Duration::from_secs(LOGIN_REMEMBER_EXPIRE_TIME)
    } else {
        Utc::now() + Duration::from_secs(LOGIN_EXPIRE_TIME)
    };

    let _ = UserOnlineModel::upsert(db, &UserOnlineModel {
        uid: user.uid,
        token_id: String::from(login_response.token.as_str()),
        token_expire: token_expire.timestamp(),
        login_at: utc_now(),
        username: user.username.to_string(),
        dpt_name: dpt.name.to_string(),
        net: String::from(info.network.as_str()),
        ip: String::from(info.ip.as_str()),
        location: String::from(info.location.as_str()),
        device: String::from(info.device.as_str()),
        browser: String::from(info.browser.as_str()),
        os: String::from(info.os.as_str()),
    }).await;

    Ok(())
}
