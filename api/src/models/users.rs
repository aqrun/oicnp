use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub uid: i32,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub status: i32,
    pub email: String,
    pub admin: bool,
    pub intro: String,
    pub last_login_on: NaiveDateTime,
    pub salt: String,
    pub must_change_password: bool,
    pub password_changed_on: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub status: i16,
    pub email: String,
    pub admin: bool,
    pub intro: String,
    pub salt: String,
    pub must_change_password: bool,
}

#[derive(Queryable)]
pub struct UserPicture {
    pub bundle: String,
    pub uid: i32,
    pub fid: i32,
    pub weight: i32,
    pub alt: String,
    pub title: String,
    pub width: i32,
    pub height: i32,
}

    
