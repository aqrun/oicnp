use crate::models::{Users, NewUser, UpdateUser, ResUserList};
use oicnp_core::{DatabaseConnection, prelude::{
    anyhow::{anyhow, Result}
}, services as core_services, models as core_models, DbConn};
use crate::typings::GqlResult;
use crate::utils::oic_err;

pub async fn find_users(
    db: &DbConn,
    page: Option<u64>,
    page_size: Option<u64>,
) -> GqlResult<ResUserList> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);

    let res = core_services::find_users(
        db, page, page_size,
    )
        .await?;

    let data = res
        .data
        .into_iter()
        .map(move |item| {
            return Users {
                user: item,
            };
        })
        .collect::<Vec<Users>>();

    let res_list_data = ResUserList {
        data,
        page_info: crate::typings::PagerInfo {
            page: res.page as i32,
            page_size: res.page_size as i32,
            total_count: res.total_count as i32,
        },
    };
    Ok(res_list_data)
}

///
/// 创建用户
///
/// 返回创建成功的用户信息
/// 
pub async fn create_user(
    db: &DatabaseConnection,
    new_user: &NewUser,
    user_id: &str,
) -> Result<Users> {
    let core_new_user = new_user.to_core_new_user();
    let uid = core_services::create_user(db, &core_new_user, user_id).await?;

    let user = match core_services::find_user_by_uid(db, &uid).await {
        Ok(user) => user,
        Err(err) => {
            let msg = format!("用户创建查询失败: {}", err.to_string());
            return Err(anyhow!(msg));
        }
    };

    let users = Users {
        user,
    };
    Ok(users)
}

///
/// 查询单个用户信息
///
pub async fn find_user(
    db: &DbConn,
    uid: Option<String>,
    username: Option<String>,
    email: Option<String>,
) -> Result<Users> {
    let mut user: Option<core_models::User> = None;
    let mut msg = String::from("");

    if uid.is_none() && username.is_none() && email.is_none() {
        return Err(anyhow!("必须指定查询信息: uid/username/email"));
    }

    if let Some(uid) = uid {
        let res_user = core_services::find_user_by_uid(db, uid.as_str()).await?;
        user = Some(res_user);
        msg = format!("uid: {}", uid.as_str());
    } else if let Some(username) = username {
        let res_user = core_services::find_user_by_username(db, username.as_str()).await?;
        user = Some(res_user);
        msg = format!("username: {}", username.as_str());
    } else if let Some(email) = email {
        let res_user = core_services::find_user_by_email(db, email.as_str()).await?;
        user = Some(res_user);
        msg = format!("email: {}", email.as_str());
    }

    if let Some(user) = user {
        let res = Users {
            user,
        };
        return Ok(res);
    }

    Err(anyhow!("用户信息不存在 {}", msg))
}

pub async fn update_user(
    db: &DbConn,
    new_user: UpdateUser,
    uid: &str,
) -> GqlResult<String> {
    let new_user = new_user.to_core_update_user();
    let res = core_services::update_user(db, &new_user, uid).await;

    match res {
        Ok(res) => Ok(res),
        Err(err) => {
            let msg = err.to_string();
            Err(oic_err("400", msg.as_str()))
        }
    }
}

pub async fn delete_user(
    db: &DbConn,
    uid: &str,
    user_id: &str,
) -> GqlResult<String> {
    let res = core_services::delete_user(db, uid, user_id).await?;
    Ok(res)
}

pub async fn remove_user(
    db: &DbConn,
    uid: &str,
    user_id: &str,
) -> GqlResult<String> {
    let res = core_services::remove_user(db, uid, user_id).await?;
    Ok(res)
}
