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

pub async fn find_user_by_id(rb: Arc<Rbatis>, uid: i32) -> Result<Users, Error> {
    return rb.fetch_by_column("uid", uid).await;
}
