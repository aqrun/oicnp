use sea_orm::*;
use anyhow::{anyhow, Result};
use sea_orm::sea_query::Expr;
use crate::typings::ListData;
use crate::prelude::*;
use crate::models::{NewUser, UpdateUser};
use crate::entities::prelude::*;
use crate::utils::{uuid, encrypt_password, generate_salt};

///
/// 查询用户列表
/// 
pub async fn find_users(
    db: &DbConn,
    page: u64,
    page_size: u64,
) -> Result<ListData<UserModel>> {
    let mut query = UserEntity::find()
        .select_only()
        .columns([
            UserColumn::Uid,
            UserColumn::Uuid,
            UserColumn::Username,
            UserColumn::Nickname,
            UserColumn::Password,
            UserColumn::Salt,
            UserColumn::Status,
            UserColumn::Email,
            UserColumn::Gender,
            UserColumn::Phone,
            UserColumn::Avatar,
            UserColumn::RoleId,
            UserColumn::DptId,
            UserColumn::Remark,
            UserColumn::IsAdmin,
            UserColumn::LastLoginIp,
            UserColumn::LastLoginAt,
            UserColumn::CreatedBy,
            UserColumn::UpdatedBy,
            UserColumn::CreatedAt,
            UserColumn::UpdatedAt,
            UserColumn::DeletedAt,
        ]);

    query = query.order_by_desc(UserColumn::CreatedAt);

    // 获取全部数据条数据
    let total = query.clone().count(db).await?;
    let pager = query
        .into_model::<UserModel>()
        .paginate(db, page_size);
    let total_pages = pager.num_pages().await?;
    let list = pager.fetch_page(page - 1).await?;

    let list_data: ListData<UserModel> = ListData {
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
    uid: i64,
) -> ModelResult<UserModel> {
    let user = UserEntity::find()
        .filter(UserColumn::Uid.eq(uid))
        .into_model::<UserModel>()
        .one(db)
        .await?;

    user.ok_or_else(|| ModelError::EntityNotFound)
}

///
/// 根据username查找用户信息
/// 
pub async fn find_user_by_username(
    db: &DbConn,
    username: &str,
) -> ModelResult<UserModel> {
    let user = UserEntity::find()
        .filter(UserColumn::Username.eq(username))
        .into_model::<UserModel>()
        .one(db)
        .await?;

    user.ok_or_else(|| ModelError::EntityNotFound)
}

///
/// 根据邮箱查找用户信息
///
pub async fn find_user_by_email(
    db: &DbConn,
    email: &str,
) -> ModelResult<UserModel> {
    let user = UserEntity::find()
        .filter(UserColumn::Email.eq(email))
        .into_model::<UserModel>()
        .one(db)
        .await?;

    user.ok_or_else(|| ModelError::EntityNotFound)
}

///
/// 创建用户
/// 返回创建的用户UID
/// 
pub async fn create_user(
    db: &DbConn,
    new_user: &NewUser,
    user_id: i64,
) -> Result<i64> {
    if let Ok(_) = find_user_by_username(db, &new_user.username.as_str()).await {
        return Err(anyhow!("用户已存在: {}", &new_user.username.as_str()));
    }

    let mut avatar = String::from("");
    let mut remark = String::from("");

    let salt = generate_salt();
    let pass = encrypt_password(salt.as_str(), &new_user.password);

    if let Some(item) = &new_user.avatar {
        avatar = item.to_owned();
    }

    if let Some(item) = &new_user.remark {
        remark = item.to_owned();
    }

    let user_uuid: String = uuid();
    let now = chrono::Utc::now();

    let user_model = UserActiveModel {
        uuid: Set(user_uuid),
        username: Set(String::from(&new_user.username)),
        nickname: Set(String::from(&new_user.nickname)),
        password: Set(String::from(pass.as_str())),
        salt: Set(String::from(salt.as_str())),
        status: Set(String::from(&new_user.status)),
        email: Set(String::from(&new_user.email)),
        gender: Set(String::from(&new_user.gender)),
        phone: Set(String::from(&new_user.phone)),
        avatar: Set(avatar),
        role_id: Set(new_user.role_id),
        dpt_id: Set(new_user.department_id),
        remark: Set(remark),
        is_admin: Set(String::from(&new_user.is_admin)),
        last_login_ip: Set(String::from(&new_user.last_login_ip)),
        last_login_at: Set(new_user.last_login_at),
        created_by: Set(user_id),
        created_at: Set(now.naive_local()),
        ..Default::default()
    };

    let res = UserEntity::insert(user_model).exec(db).await?;

    Ok(res.last_insert_id)
}

///
/// 更新用户信息
/// 
pub async fn update_user(
    db: &DbConn,
    user: &UpdateUser,
    user_id: i64,
) -> Result<String> {
    let uid = user.uid;

    let mut q = UserEntity::update_many();

    if let Some(item) = &user.uuid {
        q = q.col_expr(UserColumn::Uuid, Expr::value(item));
    }

    if let Some(item) = &user.username {
        q = q.col_expr(UserColumn::Username, Expr::value(item));
    }

    if let Some(item) = &user.nickname {
        q = q.col_expr(UserColumn::Nickname, Expr::value(item));
    }

    if let Some(item) = &user.password {
        let salt = generate_salt();
        let pass = encrypt_password(salt.as_str(), item);
        q = q.col_expr(UserColumn::Salt, Expr::value(salt));
        q = q.col_expr(UserColumn::Password, Expr::value(pass));
    }

    if let Some(item) = &user.status {
        q = q.col_expr(UserColumn::Status, Expr::value(item));
    }
    if let Some(item) = &user.email {
        q = q.col_expr(UserColumn::Email, Expr::value(item));
    }
    if let Some(item) = &user.gender {
        q = q.col_expr(UserColumn::Gender, Expr::value(item));
    }
    if let Some(item) = &user.phone {
        q = q.col_expr(UserColumn::Phone, Expr::value(item));
    }
    if let Some(item) = &user.avatar {
        q = q.col_expr(UserColumn::Avatar, Expr::value(item));
    }
    if let Some(item) = &user.role_id {
        q = q.col_expr(UserColumn::RoleId, Expr::value(*item));
    }
    if let Some(item) = &user.dpt_id {
        q = q.col_expr(UserColumn::DptId, Expr::value(*item));
    }
    if let Some(item) = &user.remark {
        q = q.col_expr(UserColumn::Remark, Expr::value(item));
    }
    if let Some(item) = &user.is_admin {
        q = q.col_expr(UserColumn::IsAdmin, Expr::value(item));
    }
    if let Some(item) = &user.created_by {
        q = q.col_expr(UserColumn::CreatedBy, Expr::value(*item));
    }
    if let Some(item) = &user.updated_by {
        q = q.col_expr(UserColumn::UpdatedBy, Expr::value(*item));
    }
    if let Some(item) = &user.created_at {
        q = q.col_expr(UserColumn::CreatedAt, Expr::value(*item));
    }

    let update_at = chrono::Local::now().naive_local();
    q = q.col_expr(UserColumn::UpdatedAt, Expr::value(update_at));
    q = q.col_expr(UserColumn::UpdatedBy, Expr::value(user_id));

    q.filter(UserColumn::Uid.eq(uid))
        .exec(db)
        .await?;

    Ok(String::from("用户更新成功"))
}

///
/// 软删除用户
/// 
pub async fn delete_user(
    db: &DbConn,
    uid: i64,
    user_id: i64,
) -> Result<String> {
    let mut q = UserEntity::update_many();

    let update_at = chrono::Local::now().naive_local();
    q = q.col_expr(UserColumn::UpdatedAt, Expr::value(update_at));
    q = q.col_expr(UserColumn::UpdatedBy, Expr::value(user_id));
    q = q.col_expr(UserColumn::DeletedAt, Expr::value(update_at));
    q.filter(UserColumn::Uid.eq(uid))
        .exec(db)
        .await?;

    Ok(String::from("用户删除成功"))
}

///
/// 清除用户
///
pub async fn remove_user(
    db: &DbConn,
    uid: i64,
    user_id: i64,
) -> Result<String> {
    let user = find_user_by_uid(db, user_id).await?;

    if !user.is_admin.as_str().eq("1") && uid != user_id {
        return Err(anyhow!("NOT_AUTHORIZED"));
    }

    UserEntity::delete_many()
        .filter(UserColumn::Uid.eq(uid))
        .exec(db)
        .await?;
    Ok(String::from("清除成功"))
}

///
/// 强制清除用户
///
pub async fn force_remove_user(
    db: &DbConn,
    uid: i64,
) -> Result<String> {
    UserEntity::delete_many()
        .filter(UserColumn::Uid.eq(uid))
        .exec(db)
        .await?;
    Ok(String::from("清除成功"))
}
