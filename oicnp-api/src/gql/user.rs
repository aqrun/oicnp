use crate::models::{Users, NewUser, ReqCtx, UpdateUser, ResUserList};
use crate::services;
use crate::utils::oic_err;
use async_graphql::{Context, Object};
use oicnp_core::DbConn;
use crate::typings::GqlResult;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    ///
    /// 获取用户信息列表
    ///
    async fn users(
        &self,
        ctx: &Context<'_>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> GqlResult<ResUserList> {
        let db = ctx.data_unchecked::<DbConn>();
        let res = services::find_users(
            db, page, page_size,
        ).await;
        res
    }

    ///
    /// 查询单个用户信息
    ///
    async fn user(
        &self,
        ctx: &Context<'_>,
        uid: Option<String>,
        username: Option<String>,
        email: Option<String>,
    ) -> GqlResult<Users> {
        let db = ctx.data_unchecked::<DbConn>();

        return match services::find_user(db, uid, username, email).await {
            Ok(user) => Ok(user),
            Err(err) => {
                let msg = err.to_string();
                Err(oic_err("400", msg.as_str()))
            },
        };
    }
}

#[derive(Default)]
pub struct UserMutations;

#[Object]
impl UserMutations {
    ///
    /// 创建新用户
    ///
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        new_user: NewUser,
    ) -> GqlResult<Users> {
        let db = ctx.data_unchecked::<DbConn>();
        let req = ctx.data_unchecked::<ReqCtx>();
        let uid = req.login_info.uid.as_str();

        return match services::create_user(db, &new_user, uid).await {
            Ok(users) => Ok(users),
            Err(err) => {
                let msg = err.to_string();
                Err(oic_err("400", msg.as_str()))
            },
        };
    }

    ///
    /// 更新单个用户信息
    ///
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        new_user: UpdateUser,
    ) -> GqlResult<String> {
        let db = ctx.data_unchecked::<DbConn>();
        let req = ctx.data_unchecked::<ReqCtx>();
        let uid = req.login_info.uid.as_str();

        let res = services::update_user(db, new_user, uid).await;

        res
    }

    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        uid: String,
    ) -> GqlResult<String> {
        let db = ctx.data_unchecked::<DbConn>();
        let req = ctx.data_unchecked::<ReqCtx>();
        let user_id = req.login_info.uid.as_str();

        let res = services::delete_user(db, uid.as_str(), user_id).await;

        res
    }

    async fn remove_user(
        &self,
        ctx: &Context<'_>,
        uid: String,
    ) -> GqlResult<String> {
        let db = ctx.data_unchecked::<DbConn>();
        let req = ctx.data_unchecked::<ReqCtx>();
        let user_id = req.login_info.uid.as_str();

        let res = services::remove_user(db, uid.as_str(), user_id).await;

        res
    }
}

