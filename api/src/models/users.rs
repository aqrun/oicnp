use async_graphql::{Object};

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub uid: i32,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub status: i32,
    pub email: String,
    pub admin: bool,
    pub intro: String,
    pub last_login_on: rbatis::DateTimeNative,
    pub salt: String,
    pub must_change_password: bool,
    pub password_changed_on: i32,
    pub created_at: rbatis::DateTimeNative,
    pub updated_at: rbatis::DateTimeNative,
}

#[Object]
impl User {
    async fn uid(&self) -> i32 {
        self.uid
    }

    async fn username(&self) -> &str {
        self.username.as_str()
    }

    async fn nickname(&self) -> &str {
        self.nickname.as_str()
    }

    async fn email(&self) -> &str {
        self.email.as_str()
    }
}

#[crud_table(table_name: user)]
#[derive(Clone, Debug)]
pub struct NewUser {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub status: i32,
    pub email: String,
    pub admin: bool,
    pub intro: String,
    pub last_login_on: rbatis::DateTimeNative,
    pub salt: String,
    pub must_change_password: bool,
    pub password_changed_on: i32,
}

#[crud_table]
#[derive(Clone, Debug)]
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

    
