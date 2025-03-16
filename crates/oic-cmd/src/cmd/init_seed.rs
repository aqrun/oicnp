use oic_core::{
    AppContext,
    entities::prelude::*,
};
use sea_orm::{ConnectionTrait, Statement};
use anyhow::Result;

///
/// 数据表最初数据填入
/// 直接操作数据表
pub async fn run(ctx: &AppContext) -> Result<()> {
    seed_roles(ctx).await?;
    // seed_users(ctx).await?;
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
    let params = vec![
        RoleModel {
            vid: Some(String::from("admin")),
            name: Some(String::from("管理员")),
            weight: Some(1),
            status: Some(String::from("1")),
            ..Default::default()
        },
        RoleModel {
            vid: Some(String::from("author")),
            name: Some(String::from("作者")),
            weight: Some(2),
            status: Some(String::from("1")),
            ..Default::default()
        },
        RoleModel {
            vid: Some(String::from("member")),
            name: Some(String::from("成员")),
            weight: Some(3),
            status: Some(String::from("1")),
            ..Default::default()
        },
        RoleModel {
            vid: Some(String::from("guest")),
            name: Some(String::from("游客")),
            weight: Some(4),
            status: Some(String::from("1")),
            ..Default::default()
        },
    ];
    let _ = RoleModel::create_multi(&ctx.db, params.as_slice()).await?;
    println!("角色数据初始化完成");
    Ok(())
}

async fn seed_users(ctx: &AppContext) -> Result<()> {
    ctx.db.execute(Statement::from_string(
        ctx.db.get_database_backend(),
        format!(r#"
            INSERT INTO users (uid, username, nickname, password, salt, is_admin, email, status)
            VALUES
            ("admin", "管理员", 0, "1"),
            ("author", "作者", 2, "1"),
            ("member", "成员", 3, "1"),
            ("guest", "游客", 4, "1")
        "#),
    )).await?;
    Ok(())
}