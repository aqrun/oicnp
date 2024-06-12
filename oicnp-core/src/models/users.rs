use crate::DateTime;
use crate::entities::prelude::*;
use async_graphql::{Object, SimpleObject, InputObject};
use crate::prelude::DateFormat;

#[Object]
impl UserModel {
    pub async fn uid(&self) -> i64 {
        self.uid
    }

    async fn username(&self) -> &str {
        self.username.as_str()
    }

    async fn nickname(&self) -> &str {
        self.nickname.as_str()
    }

    async fn salt(&self) -> &str {
        self.salt.as_str()
    }
    async fn status(&self) -> &str {
        self.status.as_str()
    }

    async fn email(&self) -> &str {
        self.email.as_str()
    }

    async fn gender(&self) -> &str {
        self.gender.as_str()
    }

    async fn phone(&self) -> &str {
        self.phone.as_str()
    }

    async fn avatar(&self) -> &str {
        self.avatar.as_str()
    }

    async fn role(&self) -> i64 {
        self.role_id
    }

    async fn department_id(&self) -> i64 {
        self.dpt_id
    }

    async fn remark(&self) -> &str {
        self.remark.as_str()
    }

    async fn is_admin(&self) -> &str {
        self.is_admin.as_str()
    }

    async fn last_login_ip(&self) -> &str {
        self.last_login_ip.as_str()
    }

    async fn last_login_at(&self) -> String {
        if let Some(login_at) = self.last_login_at {
            return login_at
                .format(&DateFormat::Normal.to_string())
                .to_string()
        }

        String::from("")
    }

    async fn created_by(&self) -> i64 {
        self.created_by
    }

    async fn updated_by(&self) -> i64 {
        self.updated_by
    }

    async fn created_at(&self) -> String {
        self.created_at
            .format(&DateFormat::Normal.to_string())
            .to_string()
    }

    async fn updated_at(&self) -> String {
        if let Some(item) = self.updated_at {
            return item
                .format(&DateFormat::Normal.to_string())
                .to_string()
        }

        String::from("")
    }

    async fn deleted_at(&self) -> String {
        if let Some(item) = self.deleted_at {
            return item
                .format(&DateFormat::Normal.to_string())
                .to_string()
        }

        String::from("")
    }
}

#[derive(Clone, Debug, Default, InputObject)]
pub struct NewUser {
    pub uuid: String,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub salt: String,
    pub status: String,
    pub email: String,
    pub gender: String,
    pub phone: String,
    pub avatar: Option<String>,
    pub role_id: i64,
    pub department_id: i64,
    pub remark: Option<String>,
    pub is_admin: String,
    pub last_login_ip: String,
    pub last_login_at: Option<DateTime>,
    pub created_by: i64,
    pub updated_by: i64,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Clone, Debug, Default, InputObject)]
pub struct UpdateUser {
    pub uid: i64,
    pub uuid: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub status: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub role_id: Option<i64>,
    pub dpt_id: Option<i64>,
    pub remark: Option<String>,
    pub is_admin: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_at: Option<DateTime>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

///
/// 带分页的用户列表数据
///
#[derive(SimpleObject, Debug, Clone)]
pub struct ResUserList {
    pub data: Vec<UserModel>,
    pub page_info: crate::typings::PagerInfo,
}
