use crate::models::{
    Files, Users, NewUser,
};
use oicnp_core::{
    DatabaseConnection,
    prelude::{
        anyhow::{anyhow, Result}
    },
    services as core_services,
    models as core_models,
};

///
/// 创建用户
///
/// 返回保存成功的UID
/// 
pub async fn create_user(
    db: &DatabaseConnection,
    new_user: &NewUser,
) -> Result<String> {
    let core_new_user = new_user.to_core_new_user();
    let res = core_services::create_user(db, &core_new_user)
        .await?;
    Ok(res)
}
