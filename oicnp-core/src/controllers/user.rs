use crate::models::{NewUser, ReqCtx, UpdateUser};
use crate::services;
use crate::{
    DbConn, services::find_user_by_uid,
    prelude::*,
    prelude::async_graphql::{Context, Object},
    typings::ListData,
};
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
    ) -> GqlResult<ListData<UserModel>> {
        let page = page.unwrap_or(0u64);
        let page_size = page_size.unwrap_or(0u64);
        let db = ctx.data_unchecked::<DbConn>();
        let res = services::find_users(
            db, page, page_size,
        ).await?;
        Ok(res)
    }

    ///
    /// 查询单个用户信息
    ///
    async fn user(
        &self,
        ctx: &Context<'_>,
        uid: Option<i64>,
        username: Option<String>,
        email: Option<String>,
    ) -> GqlResult<UserModel> {
        let db = ctx.data_unchecked::<DbConn>();

        // return match services::find_user(db, uid, username, email).await {
        //     Ok(user) => Ok(user),
        //     Err(err) => {
        //         let msg = err.to_string();
        //         Err(oic_err("400", msg.as_str()))
        //     },
        // };
        let user = find_user_by_uid(db, uid.unwrap_or(0)).await?;
        Ok(user)
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
    ) -> GqlResult<i64> {
        let db = ctx.data_unchecked::<DbConn>();
        let req = ctx.data_unchecked::<ReqCtx>();
        let uid = req.login_info.uid;

        let res = services::create_user(db, &new_user, uid).await?;
        Ok(res)
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
        let uid = req.login_info.uid;

        let res = services::update_user(db, &new_user, uid).await?;

        Ok(res)
    }

    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        uid: i64,
    ) -> GqlResult<String> {
        let db = ctx.data_unchecked::<DbConn>();
        let req = ctx.data_unchecked::<ReqCtx>();
        let user_id = req.login_info.uid;

        let res = services::delete_user(db, uid, user_id).await?;

        Ok(res)
    }

    async fn remove_user(
        &self,
        ctx: &Context<'_>,
        uid: i64,
    ) -> GqlResult<String> {
        let db = ctx.data_unchecked::<DbConn>();
        let req = ctx.data_unchecked::<ReqCtx>();
        let user_id = req.login_info.uid;

        let res = services::remove_user(db, uid, user_id).await?;

        Ok(res)
    }
}

