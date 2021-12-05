// use chrono::NaiveDateTime;

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub uid: Option<i32>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub status: Option<i32>,
    pub email: Option<String>,
    pub admin: Option<bool>,
    pub intro: Option<String>,
    pub last_login_on: Option<rbatis::DateTimeNative>,
    pub salt: Option<String>,
    pub must_change_password: Option<bool>,
    pub password_changed_on: Option<i32>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}

#[crud_table]
#[derive(Clone, Debug)]
pub struct UserPicture {
    pub bundle: Option<String>,
    pub uid: Option<i32>,
    pub fid: Option<i32>,
    pub weight: Option<i32>,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

    
