use async_graphql::{Object, Context};
use crate::gql::GqlResult;
use crate::models::Nodes;

#[derive(Default)]
pub struct NodeQuery;

#[Object]
impl NodeQuery {
    async fn nodes(
        &self,
        ctx: &Context<'_>
    ) -> GqlResult<Vec<Nodes>> {
        Ok(vec!())
    }
}