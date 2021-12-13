use async_graphql::{Object, Context};
use serde::{Serialize, Deserialize};
use crate::typings::GqlState;
use crate::services;
use crate::models::{File};

#[crud_table(table_name: users)]
#[derive(Clone, Debug, Serialize, Deserialize)]
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

    async fn status(&self) -> i32 {
        self.status
    }

    async fn admin(&self) -> bool {
        self.admin
    }

    async fn intro(&self) -> &str {
        self.intro.as_str()
    }

    async fn last_login_on(&self) -> String {
        self.last_login_on.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    async fn salt(&self) -> &str {
        self.salt.as_str()
    }

    async fn must_change_password(&self) -> bool {
        self.must_change_password
    }

    async fn password_changed_on(&self) -> i32 {
        self.password_changed_on
    }

    async fn created_at(&self) -> String {
        self.last_login_on.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    async fn updated_at(&self) -> String {
        self.last_login_on.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    async fn avatar(&self, ctx: &Context<'_>) -> File {
        let rb = ctx.data_unchecked::<GqlState>().rbatis.clone();
        let res = services::find_user_avatar(rb.clone(), &self.uid).await;

        let file = match res {
            Ok(file) => file,
            Err(err) => {
                println!("Fetch user avatar error: {}", err.to_string());
                File::default()
            },
        };

        file
    }
}

#[crud_table(table_name: users)]
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

    
