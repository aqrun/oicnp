use async_graphql::MergedObject;
use crate::controllers::{
    AuthMutations,
    NodeMutations,
    NodeQuery,
    UserMutations,
    UserQuery,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    UserQuery,
    NodeQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    AuthMutations,
    UserMutations,
    NodeMutations,
);
