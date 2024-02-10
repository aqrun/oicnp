mod auth;
pub mod node;
mod roots;
pub mod user;

pub use auth::*;
pub use node::*;
pub use roots::*;
pub use user::*;

use crate::extensions::Resp as RespExt;
use async_graphql::{
    EmptySubscription, Schema,
};
use crate::{establish_connection, DB, G};

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let db = DB.get_or_init(establish_connection).await;

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
        .data(db.clone())
        .extension(RespExt)
        .finish()
}

