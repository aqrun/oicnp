use crate::gql::GqlResult;
use crate::models::{Users, NewUser};
use crate::services;
use crate::typings::OicRes;
use crate::utils::{oic_ok, oic_err};
use async_graphql::{Context, Object};
use oicnp_core::{
    prelude::{
        anyhow::{anyhow, Result},
    },
    services as core_services,
    DbConn,
};

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

    async fn user(&self, ctx: &Context<'_>, uid: String) -> Result<Users> {
        let db = ctx.data_unchecked::<DbConn>();

        match core_services::find_user_by_uid(db, uid.as_str()).await {
            Ok(user) => {
                Ok(Users {
                    user,
                })
            },
            Err(err) => {
                Err(anyhow!("errr 用户不存在---{}", err.to_string()))
            }
        }
    }

    async fn user_test(&self, ctx: &Context<'_>, uid: String) -> Result<OicRes<Users>> {
        let db = ctx.data_unchecked::<DbConn>();

        match core_services::find_user_by_uid(db, uid.as_str()).await {
            Ok(user) => {
                oic_ok(Users {
                    user,
                })
            },
            Err(err) => {
                oic_err("400", err.to_string())
            }
        }
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
    ) -> Result<Users> {
        let db = ctx.data_unchecked::<DbConn>();
        let uid = match services::create_user(db, &new_user).await {
            Ok(uid) => uid,
            Err(err) => {
                // let msg = format!("用户创建失败: {}", err.to_string());
                // return utils::oic_result_error("400", msg.as_str());
                return Err(anyhow!(""));
            }
        };

        let user = match core_services::find_user_by_uid(db, &uid).await {
            Ok(user) => user,
            Err(err) => {
                // return utils::oic_result_error("400", err.to_string().as_str());
                return Err(anyhow!(""));
            }
        };
        
        let users = Users {
            user,
        };

        // let res = utils::oic_result_success(users);
        Ok(users)
    }
}

