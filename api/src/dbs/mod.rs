use crate::services::G;
use rbatis::rbatis::Rbatis;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;

pub async fn establish_connection() -> Rbatis {
    let url = &G.config.database_url;
    let rb = Rbatis::new();
    rb.link(url)
        .await
        .expect(&format!("Error connecting to {}", url));
    rb
}

pub async fn init_rbatis() -> Rbatis {
    let url = &G.config.database_url;
    let mut rb = Rbatis::new();
    // logic plugin 设置逻辑删除插件
    rb.logic_plugin = Some(Box::new(
        RbatisLogicDeletePlugin::new_opt(
            &G.config.logic_column,
            &G.config.logic_deleted as i32,
            &G.config.logic_un_deleted as i32
        )
    ));

    if &G.config.debug.eq(&false) && &rb.is_debug_mode() {
        panic!(r#"已使用 release 模式，但 rbatis 仍使用 debug 模式！请删除
        Cargo.toml 中 rbatis 的配置 features = ["debug_mode"]"#);
    }

    println!(
        "[oicnp] Rbatis link database({})...",
        url[0..url.find(":").unwrap_or(0)]
    );
    rb
        .link(url)
        .await
        .expect("[oicnp] Rbatis link database failed!");
    println!("[oicnp] Rbatis link database success!");
    rb
}
