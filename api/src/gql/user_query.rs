use async_graphql::{Object, Context};
use rbatis::crud::CRUD;
use rbatis::Error;
use crate::gql::GqlResult;
use crate::typings::{State};
use crate::models::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn hero(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<String> {
        Ok("".to_string())
    }

    async fn users(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<Vec<User>> {
        let state = ctx.data::<State>().expect("State Error");
        let rb = state.rbatis.clone();
        let res: Result<Vec<User>, Error> = rb.fetch_list().await;

        if let Ok(res) = res {
            return Ok(res);
        }

        return Ok(vec!())
    }
}