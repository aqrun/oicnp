use async_graphql::{Enum};
use serde::{Deserialize, Serialize};

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

///
/// 星战系列
/// 正传三部曲
#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Episode {
    // 新希望
    NewHope,
    // 帝国反击战
    Empire,
    // 绝地归来
    Jedi
}