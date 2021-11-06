use async_graphql::{Object, Context, FieldResult};
use crate::typings::{GqlResult};
use async_graphql::connection::{Connection, EmptyFields};
use crate::services;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hero(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<String> {
       Ok("".to_string())
    }
}