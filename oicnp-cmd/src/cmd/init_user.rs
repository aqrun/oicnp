use oicnp_api::{
    services::{create_user, find_user},
    models::NewUser,
};
use oicnp_core::{
    DB, establish_connection,
    services::force_remove_user,
};

///
/// 创建初始用户数据
/// 
pub async fn run() {
    let db = DB.get_or_init(establish_connection).await;
    // 当前登陆的用户
    let current_user_id = "";

    // 先清除老的数据
    let old_user = find_user(
        db, None, Some(String::from("aqrun")), None
    ).await;

    if let Ok(user) = old_user {
        let res = force_remove_user(
            db, &user.user.uid
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
        nickname: Some(String::from("子十")),
        password: String::from("123456"),
        salt: None,
        status: String::from("1"),
        email: String::from("aqrun@sina.com"),
        gender: Some(String::from("1")),
        phone: None,
        avatar: None,
        role_id: None,
        department_id: None,
        remark: None,
        is_admin: Some(String::from("1")),
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