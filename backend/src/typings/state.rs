use async_graphql::{Schema, EmptySubscription};
use crate::gql::{QueryRoot, MutationRoot};

#[derive(Clone)]
pub struct State {
    pub schema: Schema<
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    >,
}