use oicnp_core::{
    establish_connection, 
    services::{create_user, find_user_by_username, force_remove_user},
    DB,
    models::NewUser,
};

///
/// 创建初始用户数据
/// 
pub async fn run() {
    let db = DB.get_or_init(establish_connection).await;
    // 当前登陆的用户
    let current_user_id = 0i64;

    // 先清除老的数据
    let old_user = find_user_by_username(
        db, "aqrun"
    ).await;

    if let Ok(user) = old_user {
        let res = force_remove_user(
            db, user.uid
        ).await;

        match res {
            Ok(_) => {
                println!("用户删除: {:?}", "aqrun");
            },
            Err(err) => {
                println!("用户删除失败: {:?}", err);
            }
        };
    }

    let new_user = NewUser {
        username: String::from("aqrun"),
        nickname: String::from("子十"),
        password: String::from("123456"),
        salt: String::from(""),
        status: String::from("1"),
        email: String::from("aqrun@sina.com"),
        gender: String::from("1"),
        is_admin: String::from("1"),
        ..Default::default()
    };

    let res = create_user(db, &new_user, current_user_id).await;

    match res {
        Ok(_) => {
            println!("用户创建成功: {:?}", "aqrun");
        },
        Err(err) => {
            println!("用户创建失败: {:?}", err);
        }
    };
}