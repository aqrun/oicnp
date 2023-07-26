use async_graphql::{Schema, EmptySubscription};
use crate::gql::{QueryRoot, MutationRoot};
use crate::models::ReqCtx;

#[derive(Clone)]
pub struct State {
    pub schema: Schema<
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    >,
    pub req_ctx: Option<ReqCtx>,
}
