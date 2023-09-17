use sea_orm::*;
use anyhow::{anyhow, Result};
use crate::typings::ListData;
use crate::prelude::*;
use crate::models::{NewUser, User};
use crate::entities::{
    sys_users,
    prelude::{SysUsers},
};
use crate::utils::uuid;

///
/// 查询用户列表
/// 
pub async fn find_users() -> ListData<()>{
    ListData {
        data: Vec::new(),
        page: 1,
        page_size: 10,
        total_pages: 100,
        total_count: 100,
    }
}

///
/// 查找用户信息
/// 
pub async fn find_user_by_username(
    db: &DbConn,
    username: &str,
) -> Result<User> {
    let mut q = SysUsers::find();
    q = q.filter(sys_users::Column::Username.eq(username));

    let res = q.into_model::<User>().one(db).await?;

    if let Some(user) = res {
        return Ok(user);
    }

    Err(anyhow!("User not exist: {}", username))
}

///
/// 创建用户
/// 
pub async fn create_user(
    db: &DbConn,
    new_user: &NewUser,
) -> Result<User> {
    if let Ok(user) = find_user_by_username(db, &new_user.username.as_str()).await {
        return Ok(user);
    }

    let mut avatar: Option<String> = None;
    let mut role_id: Option<String> = None;
    let mut department_id: Option<String> = None;
    let mut remark: Option<String> = None;

    if let Some(item) = &new_user.avatar {
        avatar = Some(item.to_owned());
    }

    if let Some(item) = &new_user.role_id {
        role_id = Some(item.to_owned());
    }

    if let Some(item) = &new_user.department_id {
        department_id = Some(item.to_owned());
    }

    if let Some(item) = &new_user.remark {
        remark = Some(item.to_owned());
    }

    let _ = sys_users::ActiveModel {
        uid: Set(uuid()),
        username: Set(Some(String::from(&new_user.username))),
        nickname: Set(Some(String::from(&new_user.nickname))),
        password: Set(Some(String::from(&new_user.password))),
        salt: Set(Some(String::from(&new_user.salt))),
        status: Set(Some(String::from(&new_user.status))),
        email: Set(Some(String::from(&new_user.email))),
        gender: Set(Some(String::from(&new_user.gender))),
        phone: Set(Some(String::from(&new_user.phone))),
        avatar: Set(avatar),
        role_id: Set(role_id),
        department_id: Set(department_id),
        remark: Set(remark),
        is_admin: Set(Some(String::from(&new_user.is_admin))),
        last_login_ip: Set(Some(String::from(&new_user.last_login_ip))),
        last_login_at: Set(new_user.last_login_at),
        created_by: Set(String::from(&new_user.created_by)),
        updated_by: Set(Some(String::from(&new_user.updated_by))),
        created_at: Set(new_user.created_at),
        updated_at: Set(new_user.updated_at),
        deleted_at: Set(new_user.deleted_at),
        ..Default::default()
    };

    if let Ok(user) = find_user_by_username(db, &new_user.username.as_str()).await {
        return Ok(user);
    }

    Err(anyhow!("User save failed: {}", &new_user.username))
}

///
/// 更新用户信息
/// 
pub async fn update_user() {

}

///
/// 删除用户
/// 
pub async fn delete_user() {

}