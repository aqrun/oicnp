use async_graphql::{Object};
use crate::typings::GqlResult;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn add(&self) -> GqlResult<usize> {
        Ok(0usize)
    }
}
