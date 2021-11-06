use async_graphql::{Enum};
use serde::{Deserialize, Serialize};

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;