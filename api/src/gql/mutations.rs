use async_graphql::{Object};
use crate::gql::GqlResult;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn add(&self) -> GqlResult<usize> {
        Ok(0usize)
    }
}
