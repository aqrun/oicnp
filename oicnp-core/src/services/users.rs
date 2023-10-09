use sea_orm::*;
use anyhow::{anyhow, Result};
use sea_orm::sea_query::Expr;
use crate::typings::ListData;
use crate::prelude::*;
use crate::models::{NewUser, User, UpdateUser};
use crate::entities::{
    sys_users,
    prelude::{SysUsers},
};
use crate::utils::{uuid, encrypt_password, generate_salt};
use chrono::prelude::NaiveDateTime;

///
/// 查询用户列表
/// 
pub async fn find_users(
    db: &DbConn,
    page: u64,
    page_size: u64,
) -> Result<ListData<User>> {
    let mut query = SysUsers::find()
        .select_only()
        .columns([
            sys_users::Column::Uid,
            sys_users::Column::Username,
            sys_users::Column::Nickname,
            sys_users::Column::Password,
            sys_users::Column::Salt,
            sys_users::Column::Status,
            sys_users::Column::Email,
            sys_users::Column::Gender,
            sys_users::Column::Phone,
            sys_users::Column::Avatar,
            sys_users::Column::RoleId,
            sys_users::Column::DepartmentId,
            sys_users::Column::Remark,
            sys_users::Column::IsAdmin,
            sys_users::Column::LastLoginIp,
            sys_users::Column::LastLoginAt,
            sys_users::Column::CreatedBy,
            sys_users::Column::UpdatedBy,
            sys_users::Column::CreatedAt,
            sys_users::Column::UpdatedAt,
            sys_users::Column::DeletedAt,
        ]);

    query = query.order_by_desc(sys_users::Column::CreatedAt);

    // 获取全部数据条数据
    let total = query.clone().count(db).await?;
    let pager = query
        .into_model::<User>()
        .paginate(db, page_size);
    let total_pages = pager.num_pages().await?;
    let list = pager.fetch_page(page - 1).await?;

    let list_data: ListData<User> = ListData {
        data: list,
        page,
        page_size,
        total_pages,
        total_count: total,
    };

    Ok(list_data)
}

///
/// 根据用户ID查找用户信息
///
pub async fn find_user_by_uid(
    db: &DbConn,
    uid: &str,
) -> Result<User> {
    let mut q = SysUsers::find();
    q = q.filter(sys_users::Column::Uid.eq(uid));

    let res = q.into_model::<User>().one(db).await?;

    if let Some(user) = res {
        return Ok(user);
    }

    Err(anyhow!("用户不存在uid: {}", uid))
}

///
/// 根据username查找用户信息
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

    Err(anyhow!("用户不存在username: {}", username))
}

///
/// 根据邮箱查找用户信息
///
pub async fn find_user_by_email(
    db: &DbConn,
    email: &str,
) -> Result<User> {
    let mut q = SysUsers::find();
    q = q.filter(sys_users::Column::Email.eq(email));

    let res = q.into_model::<User>().one(db).await?;

    if let Some(user) = res {
        return Ok(user);
    }

    Err(anyhow!("用户不存在email: {}", email))
}

///
/// 创建用户
/// 返回创建的用户UID
/// 
pub async fn create_user(
    db: &DbConn,
    new_user: &NewUser,
    user_id: &str,
) -> Result<String> {
    if let Ok(user) = find_user_by_username(db, &new_user.username.as_str()).await {
        return Err(anyhow!("用户已存在: {}", &new_user.username.as_str()));
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

    let uid = uuid();

    let user_model = sys_users::ActiveModel {
        uid: Set(String::from(uid.as_str())),
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
        created_by: Set(String::from(user_id)),
        updated_by: Set(Some(String::from(""))),
        created_at: Set(new_user.created_at),
        updated_at: Set(None),
        deleted_at: Set(new_user.deleted_at),
        ..Default::default()
    };

    let res = SysUsers::insert(user_model).exec(db).await?;

    if let InsertResult { last_insert_id } = res {
        return Ok(last_insert_id);
    }

    return Err(anyhow!("用户保存失败：{:?}", &new_user.username));
}

///
/// 更新用户信息
/// 
pub async fn update_user(
    db: &DbConn,
    user: &UpdateUser,
    user_id: &str,
) -> Result<String> {
    let uid = &user.uid;

    if uid.is_empty() {
        return Err(anyhow!("uid 不能为空"));
    }

    let mut q = SysUsers::update_many();

    if let Some(item) = &user.username {
        q = q.col_expr(sys_users::Column::Username, Expr::value(item));
    }

    if let Some(item) = &user.nickname {
        q = q.col_expr(sys_users::Column::Nickname, Expr::value(item));
    }

    if let Some(item) = &user.password {
        let salt = generate_salt();
        let pass = encrypt_password(salt.as_str(), item);
        q = q.col_expr(sys_users::Column::Salt, Expr::value(salt));
        q = q.col_expr(sys_users::Column::Password, Expr::value(pass));
    }

    if let Some(item) = &user.status {
        q = q.col_expr(sys_users::Column::Status, Expr::value(item));
    }
    if let Some(item) = &user.email {
        q = q.col_expr(sys_users::Column::Email, Expr::value(item));
    }
    if let Some(item) = &user.gender {
        q = q.col_expr(sys_users::Column::Gender, Expr::value(item));
    }
    if let Some(item) = &user.phone {
        q = q.col_expr(sys_users::Column::Phone, Expr::value(item));
    }
    if let Some(item) = &user.avatar {
        q = q.col_expr(sys_users::Column::Avatar, Expr::value(item));
    }
    if let Some(item) = &user.role_id {
        q = q.col_expr(sys_users::Column::RoleId, Expr::value(item));
    }
    if let Some(item) = &user.department_id {
        q = q.col_expr(sys_users::Column::DepartmentId, Expr::value(item));
    }
    if let Some(item) = &user.remark {
        q = q.col_expr(sys_users::Column::Remark, Expr::value(item));
    }
    if let Some(item) = &user.is_admin {
        q = q.col_expr(sys_users::Column::IsAdmin, Expr::value(item));
    }
    if let Some(item) = &user.created_by {
        q = q.col_expr(sys_users::Column::CreatedBy, Expr::value(item));
    }
    if let Some(item) = &user.updated_by {
        q = q.col_expr(sys_users::Column::UpdatedBy, Expr::value(item));
    }
    if let Some(item) = &user.created_at {
        q = q.col_expr(sys_users::Column::CreatedAt, Expr::value(*item));
    }

    let update_at = chrono::Local::now().naive_local();
    q = q.col_expr(sys_users::Column::UpdatedAt, Expr::value(update_at));
    q = q.col_expr(sys_users::Column::UpdatedBy, Expr::value((user_id)));
    q.filter(sys_users::Column::Uid.eq(uid.as_str()))
        .exec(db)
        .await?;

    Ok(String::from("用户更新成功"))
}

///
/// 软删除用户
/// 
pub async fn delete_user(
    db: &DbConn,
    uid: &str,
    user_id: &str,
) -> Result<String> {
    let mut q = SysUsers::update_many();

    let update_at = chrono::Local::now().naive_local();
    q = q.col_expr(sys_users::Column::UpdatedAt, Expr::value(update_at));
    q = q.col_expr(sys_users::Column::UpdatedBy, Expr::value((user_id)));
    q = q.col_expr(sys_users::Column::DeletedAt, Expr::value(update_at));
    q.filter(sys_users::Column::Uid.eq(uid.as_str()))
        .exec(db)
        .await?;

    Ok(String::from("用户删除成功"))
}

///
/// 清除用户
///
pub async fn remove_user(
    db: &DbConn,
    uid: &str,
    user_id: &str,
) -> Result<String> {
    let user = find_user_by_uid(db, user_id).await?;

    if !user.is_admin.as_str().eq("1") && !uid.eq(user_id) {
        return Err(anyhow!("NOT_AUTHORIZED"));
    }

    SysUsers::delete_many()
        .filter(sys_users::Column::Uid.eq(uid))
        .exec(db)
        .await?;
    Ok(String::from("清除成功"))
}