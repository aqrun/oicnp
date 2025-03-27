use oic_core::{
    AppContext,
    entities::prelude::*,
    models::{
        users::CreateUserReqParams,
        roles::CreateRoleReqParams,
    },
};
use anyhow::Result;
use std::fs::File;

///
/// 数据表最初数据填入
/// 直接操作数据表
pub async fn run(ctx: &AppContext) -> Result<()> {
    seed_roles(ctx).await?;
    seed_users(ctx).await?;
    Ok(())
}

/// 初始化角色信息
/// admin author member guest
/// 
/// vid: admin
/// name: 管理员
/// weight: 0
/// status: 1
async fn seed_roles(ctx: &AppContext) -> Result<()> {
    let seed_name = "role";

    // 种子文件
    let seed_file = format!("src/fixtures/{seed_name}s.yaml");
    let seed_data: Vec<CreateRoleReqParams> = serde_yaml::from_reader(File::open(seed_file)?)?;

    let _ = RoleModel::create_multi(&ctx.db, seed_data.as_slice()).await?;
    println!("角色数据初始化完成");
    Ok(())
}

async fn seed_users(ctx: &AppContext) -> Result<()> {
    let seed_name = "user";

    // 种子文件
    let seed_file = format!("src/fixtures/{seed_name}s.yaml");
    let seed_data: Vec<CreateUserReqParams> = serde_yaml::from_reader(File::open(seed_file)?)?;

    let _ = UserModel::create_multi(&ctx.db, seed_data.as_slice()).await?;
    println!("用户数据初始化完成");
    Ok(())
}