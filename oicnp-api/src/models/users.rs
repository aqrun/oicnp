use async_graphql::{Object, Context};
use crate::typings::{DateFormat};
use crate::services;
use crate::models::{Files};
use oicnp_core::{
    DateTime, DatabaseConnection,
    entities::{
        cms_nodes,
    },
    prelude::{
        anyhow::{anyhow, Result},
        chrono::prelude::*,
    }
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    pub uid: String,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub status: i32,
    pub email: String,
    pub admin: bool,
    pub intro: String,
    pub last_login_on: DateTime,
    pub salt: String,
    pub must_change_password: bool,
    pub password_changed_on: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[Object]
impl Users {
    async fn uid(&self) -> &str {
        self.uid.as_str()
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
        self.last_login_on.format(&DateFormat::Normal.to_string()).to_string()
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
        self.last_login_on.format(&DateFormat::Normal.to_string()).to_string()
    }

    async fn updated_at(&self) -> String {
        self.last_login_on.format(&DateFormat::Normal.to_string()).to_string()
    }

    async fn avatar(&self, ctx: &Context<'_>) -> Option<Files> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        // let res = services::find_user_avatar(rb.clone(), &self.uid).await;

        // let file = match res {
        //     Ok(file) => Some(file),
        //     Err(err) => {
        //         println!("Fetch user avatar error: {}", err.to_string());
        //         None
        //     },
        // };

        // file
        None
    }
}

#[derive(Clone, Debug)]
pub struct NewUser {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub status: i32,
    pub email: String,
    pub admin: bool,
    pub intro: String,
    pub last_login_on: DateTime,
    pub salt: String,
    pub must_change_password: bool,
    pub password_changed_on: i32,
}

#[derive(Clone, Debug)]
pub struct UserPictures {
    pub bundle: String,
    pub uid: String,
    pub fid: String,
    pub weight: i32,
    pub alt: String,
    pub title: String,
    pub width: i32,
    pub height: i32,
}

    
