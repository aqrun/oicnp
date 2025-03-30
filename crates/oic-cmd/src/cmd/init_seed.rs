use oic_core::{
    ModelCrudHandler,
    AppContext,
    entities::prelude::*,
};
use serde::Deserialize;
use anyhow::Result;
use std::fs::File;

///
/// 数据表最初数据填入
/// 直接操作数据表
pub async fn run(ctx: &AppContext) -> Result<()> {
    handle_seed::<RoleModel>(ctx, "role").await?;
    handle_seed::<UserModel>(ctx, "user").await?;
    Ok(())
}

/// 
/// 批量数据插入统一处理
/// 
/// @param ctx 上下文
/// @param seed_name 种子文件名称 和文件路径绑定
async fn handle_seed<TModel>(
    ctx: &AppContext,
    seed_name: &str,
) -> Result<()>
where 
    // 限定 TModel 泛型必须实现 ModelCrudHandler trait
    TModel: ModelCrudHandler,  
    // TModel::CreateReqParams 的 Deserialize 生命周期注解
    for<'de> TModel::CreateReqParams: Deserialize<'de>, 
{
    // 种子文件路径
    let seed_file = format!("src/fixtures/{seed_name}s.yaml");
    // 读取 yaml 文件并解析为指定的类型
    let seed_data: Vec<TModel::CreateReqParams> = serde_yaml::from_reader(File::open(seed_file)?)?;

    // 批量插入数据
    let _ = TModel::create_multi(&ctx.db, seed_data.as_slice()).await?;
    println!("{}数据初始化完成", seed_name);
    Ok(())
}