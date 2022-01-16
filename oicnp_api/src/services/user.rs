use crate::models::{
    Files, Users,
};
use std::sync::Arc;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbatis::py_sql;

#[py_sql("SELECT f.* FROM file f
 LEFT JOIN user_picture u
 ON f.fid = u.fid
 WHERE u.uid = #{uid}
 AND u.bundle = 'avatar'")]
pub async fn find_user_avatar(
    rb: Arc<Rbatis>,
    uid: &i32,
) -> Result<Files, Error> {
    todo!()
}

pub async fn find_user_by_id(rb: Arc<Rbatis>, uid: i32) -> Result<Users, String> {
    let res: Result<Option<Users>, Error> = rb.fetch_by_column("uid", uid).await;

    if let Ok(user) = res {
        if let Some(user) = user {
            return Ok(user);
        }
    }
    Err(format!("User not exist: {}", uid))
}

pub async fn find_user_by_username(rb: Arc<Rbatis>, username: &str) -> Result<Users, String> {
    let res: Result<Option<Users>, Error> = rb.fetch_by_column("username", username).await;
    if let Ok(user) = res {
        if let Some(user) = user {
            return Ok(user);
        }
    }
    Err(format!("User not exist: {}", username))
}

pub async fn find_user_by_email(rb: Arc<Rbatis>, email: &str) -> Result<Users, String> {
    let res: Result<Option<Users>, Error> = rb.fetch_by_column("email", email).await;
    if let Ok(user) = res {
        if let Some(user) = user {
            return Ok(user);
        }
    }
    Err(format!("User not exist: {}", email))
}
