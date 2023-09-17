use crate::models::{
    Files, Users,
};
use oicnp_core::{
    DatabaseConnection,
    entities::{
        cms_nodes,
    },
    prelude::{
        anyhow::{anyhow, Result}
    }
};

/**
 * 创建用户
 */
pub async fn create_user() {

}

// #[py_sql("SELECT f.* FROM file f
//  LEFT JOIN user_picture u
//  ON f.fid = u.fid
//  WHERE u.uid = #{uid}
//  AND u.bundle = 'avatar'")]
pub async fn find_user_avatar(
    db: &DatabaseConnection,
    uid: &i32,
) -> Result<Files> {
    todo!()
}

pub async fn find_user_by_id(db: &DatabaseConnection, uid: i32) -> Result<Users> {
    // let res: Result<Option<Users>, Error> = rb.fetch_by_column("uid", uid).await;

    // if let Ok(user) = res {
    //     if let Some(user) = user {
    //         return Ok(user);
    //     }
    // }
    Err(anyhow!("User not exist: {}", uid))
}

pub async fn find_user_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<Users> {
    // let res: Result<Option<Users>, Error> = rb.fetch_by_column("username", username).await;
    // if let Ok(user) = res {
    //     if let Some(user) = user {
    //         return Ok(user);
    //     }
    // }
    Err(anyhow!("User not exist: {}", username))
}

pub async fn find_user_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Users> {
    // let res: Result<Option<Users>, Error> = rb.fetch_by_column("email", email).await;
    // if let Ok(user) = res {
    //     if let Some(user) = user {
    //         return Ok(user);
    //     }
    // }
    Err(anyhow!("User not exist: {}", email))
}
