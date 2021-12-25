use async_graphql::{Object, Context};
use rbatis::crud::CRUD;
use rbatis::Error;
use crate::gql::GqlResult;
use crate::typings::GqlState;
use crate::models::Users;
use crate::services;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<Vec<Users>> {
        let rb = ctx.data_unchecked::<GqlState>().rbatis.clone();
        let res = rb.fetch_list::<Users>().await;

        if let Ok(res) = res {
            return Ok(res);
        }

        Ok(vec!())
    }

    async fn user(
        &self,
        ctx: &Context<'_>,
        uid: i32,
    ) -> Result<Users, String> {
        let rb = ctx.data_unchecked::<GqlState>().rbatis.clone();
        let res = services::find_user_by_id(rb.clone(), uid).await;
        res
    }
}