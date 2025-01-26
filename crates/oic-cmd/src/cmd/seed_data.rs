use std::fs::File;
use loco_rs::Result;
use oic_core::{
    models::{
        notes::CreateNoteReqParams,
        users::CreateUserReqParams,
    },
    AppContext,
};
use crate::utils::{post, r};

pub async fn run(ctx: &AppContext) -> Result<()> {
    // seed_notes(ctx).await?;
    // seed_users(ctx).await?;
    seed_menus(ctx).await?;
    Ok(())
}

///
/// note 种子
/// 
async fn seed_notes(ctx: &AppContext) -> Result<()> {
    let seed_name = "note";

    // 种子文件
    let seed_file = format!("src/fixtures/{seed_name}s.yaml");
    // 接口API
    let uri = format!("/v1/{seed_name}/add-multi");
    let seed_data: Vec<CreateNoteReqParams> = serde_yaml::from_reader(File::open(seed_file)?)?;
    let url = r(ctx, uri.as_str());
    // 请求接口
    let res = post(url.as_str(), &seed_data).await?;

    if res.is_success() {
        println!("({seed_name})添加成功: {:?}", res);
    } else {
        println!("({seed_name})添加失败: {}, {}", res.get_code(), res.get_msg());
    }

    Ok(())
}

///
/// users 种子
/// 
async fn seed_users(ctx: &AppContext) -> Result<()> {
    let seed_name = "user";

    // 种子文件
    let seed_file = format!("src/fixtures/{seed_name}s.yaml");
    // 接口API
    let uri = format!("/v1/{seed_name}/add-multi");
    let seed_data: Vec<CreateUserReqParams> = serde_yaml::from_reader(File::open(seed_file)?)?;
    let url = r(ctx, uri.as_str());
    // 请求接口
    let res = post(url.as_str(), &seed_data).await?;

    if res.is_success() {
        println!("({seed_name})添加成功: {:?}", res);
    } else {
        println!("({seed_name})添加失败: {}, {}", res.get_code(), res.get_msg());
    }

    Ok(())
}

///
/// menus 种子
/// 
async fn seed_menus(ctx: &AppContext) -> Result<()> {
    let seed_name = "menu";

    // 种子文件
    let seed_file = format!("src/fixtures/{seed_name}s.yaml");
    // 接口API
    let uri = format!("/v1/{seed_name}/add-multi");
    let seed_data: Vec<CreateUserReqParams> = serde_yaml::from_reader(File::open(seed_file)?)?;
    let url = r(ctx, uri.as_str());
    // 请求接口
    let res = post(url.as_str(), &seed_data).await?;

    if res.is_success() {
        println!("({seed_name})添加成功: {:?}", res);
    } else {
        println!("({seed_name})添加失败: {}, {}", res.get_code(), res.get_msg());
    }

    Ok(())
}
