use crate::gql::{AuthMutations, NodeMutations, NodeQuery, UserMutations, UserQuery};
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery, NodeQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(AuthMutations, UserMutations, NodeMutations);
