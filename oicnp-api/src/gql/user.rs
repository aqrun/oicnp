use crate::gql::GqlResult;
use crate::models::{Users, NewUser};
use crate::services;
use crate::utils;
use async_graphql::{Context, Object};
use oicnp_core::{
    prelude::{
        anyhow::{Result},
    },
    services as core_services,
    DbConn,
};
use crate::typings::OicResult;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> GqlResult<Vec<Users>> {
        let db = ctx.data_unchecked::<DbConn>();
        // let res = rb.fetch_list::<Users>().await;

        // if let Ok(res) = res {
        //     return Ok(res);
        // }

        Ok(vec![])
    }

    async fn user(&self, ctx: &Context<'_>, uid: i32) -> Result<Users, String> {
        let db = ctx.data_unchecked::<DbConn>();
        // let res = services::find_user_by_id(rb.clone(), uid).await;
        // res
        Err(String::from("test"))
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
    ) -> OicResult<Users> {
        let db = ctx.data_unchecked::<DbConn>();
        let uid = match services::create_user(db, &new_user).await {
            Ok(uid) => uid,
            Err(err) => {
                let msg = format!("用户创建失败: {}", err.to_string());
                return utils::oic_result_error("400", msg.as_str());
            }
        };

        let user = match core_services::find_user_by_uid(db, &uid).await {
            Ok(user) => user,
            Err(err) => {
                return utils::oic_result_error("400", err.to_string().as_str());
            }
        };
        
        let users = Users {
            user,
        };

        let res = utils::oic_result_success(users);
        res
    }
}
