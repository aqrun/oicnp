use crate::typings::DateFormat;
use async_graphql::{InputObject, Object, SimpleObject};
use oicnp_core::{
    models as core_models,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    pub user: core_models::User,
}

#[Object]
impl Users {
    async fn uid(&self) -> &str {
        self.user.uid.as_str()
    }

    async fn username(&self) -> &str {
        self.user.username.as_str()
    }

    async fn nickname(&self) -> &str {
        self.user.nickname.as_str()
    }

    async fn salt(&self) -> &str {
        self.user.salt.as_str()
    }
    async fn status(&self) -> &str {
        self.user.status.as_str()
    }

    async fn email(&self) -> &str {
        self.user.email.as_str()
    }

    async fn gender(&self) -> &str {
        self.user.gender.as_str()
    }

    async fn phone(&self) -> &str {
        self.user.phone.as_str()
    }

    async fn avatar(&self) -> String {
        if let Some(item) = &self.user.avatar {
            return String::from(item);
        }
        String::from("")
    }

    async fn role(&self) -> String {
        if let Some(item) = &self.user.role_id {
            return String::from(item);
        }
        String::from("")
    }

    async fn department_id(&self) -> String {
        if let Some(item) = &self.user.department_id {
            return String::from(item);
        }
        String::from("")
    }

    async fn remark(&self) -> String {
        if let Some(item) = &self.user.remark {
            return String::from(item);
        }
        String::from("")
    }

    async fn is_admin(&self) -> &str {
        self.user.is_admin.as_str()
    }

    async fn last_login_ip(&self) -> &str {
        self.user.last_login_ip.as_str()
    }

    async fn last_login_at(&self) -> String {
        if let Some(login_at) = self.user.last_login_at {
            return login_at
                .format(&DateFormat::Normal.to_string())
                .to_string()
        }

        String::from("")
    }

    async fn created_by(&self) -> &str {
        self.user.created_by.as_str()
    }

    async fn updated_by(&self) -> &str {
        self.user.updated_by.as_str()
    }

    async fn created_at(&self) -> String {
        self.user.created_at
            .format(&DateFormat::Normal.to_string())
            .to_string()
    }

    async fn updated_at(&self) -> String {
        if let Some(item) = self.user.updated_at {
            return item
                .format(&DateFormat::Normal.to_string())
                .to_string()
        }

        String::from("")
    }

    async fn deleted_at(&self) -> String {
        if let Some(item) = self.user.deleted_at {
            return item
                .format(&DateFormat::Normal.to_string())
                .to_string()
        }

        String::from("")
    }
}

#[derive(Clone, Debug, Default, InputObject)]
pub struct NewUser {
    pub username: String,
    pub nickname: Option<String>,
    pub password: String,
    pub salt: Option<String>,
    pub status: String,
    pub email: String,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub role_id: Option<String>,
    pub department_id: Option<String>,
    pub remark: Option<String>,
    pub is_admin: Option<String>,
}

impl NewUser {
    ///
    /// 转为 core::NewUser
    ///
    pub fn to_core_new_user(&self) -> core_models::NewUser {
        let mut nickname = String::from("");
        let mut salt = String::from("");
        let mut gender = String::from("");
        let mut phone = String::from("");
        let mut avatar = String::from("");
        let mut role_id = String::from("");
        let mut department_id = String::from("");
        let mut remark = String::from("");
        let mut is_admin = String::from("");

        if let Some(item) = &self.nickname {
            nickname = String::from(item);
        }

        if let Some(item) = &self.salt {
            salt = String::from(item);
        }

        if let Some(item) = &self.gender {
            gender = String::from(item);
        }

        if let Some(item) = &self.phone {
            phone = String::from(item);
        }

        if let Some(item) = &self.avatar {
            avatar = String::from(item);
        }

        if let Some(item) = &self.role_id {
            role_id = String::from(item);
        }
        if let Some(item) = &self.remark {
            remark = String::from(item);
        }

        if let Some(item) = &self.is_admin {
            is_admin = String::from(item);
        }

        if let Some(item) = &self.department_id {
            department_id = String::from(item);
        }

        core_models::NewUser {
            username: String::from(self.username.as_str()),
            nickname,
            password: String::from(self.password.as_str()),
            salt,
            status: String::from(self.status.as_str()),
            email: String::from(self.email.as_str()),
            gender,
            phone,
            avatar: Some(avatar),
            role_id: Some(role_id),
            department_id: Some(department_id),
            remark: Some(remark),
            is_admin,
            ..core_models::NewUser::default()
        }
    }
}

#[derive(Clone, Debug, Default, InputObject)]
pub struct UpdateUser {
    pub uid: String,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub status: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub role_id: Option<String>,
    pub department_id: Option<String>,
    pub remark: Option<String>,
    pub is_admin: Option<String>,
}

impl UpdateUser {
    pub fn to_core_update_user(&self) -> core_models::UpdateUser {
        let mut username: Option<String> = None;
        let mut nickname: Option<String> = None;
        let mut password: Option<String> = None;
        let mut status: Option<String> = None;
        let mut email: Option<String> = None;
        let mut gender: Option<String> = None;
        let mut phone: Option<String> = None;
        let mut avatar: Option<String> = None;
        let mut role_id: Option<String> = None;
        let mut department_id: Option<String> = None;
        let mut remark: Option<String> = None;
        let mut is_admin: Option<String> = None;

        if let Some(item) = &self.username {
            username = Some(String::from(item));
        }
        if let Some(item) = &self.nickname {
            nickname = Some(String::from(item));
        }
        if let Some(item) = &self.password {
            password = Some(String::from(item));
        }
        if let Some(item) = &self.status {
            status = Some(String::from(item));
        }
        if let Some(item) = &self.email {
            email = Some(String::from(item));
        }
        if let Some(item) = &self.gender {
            gender = Some(String::from(item));
        }
        if let Some(item) = &self.phone {
            phone = Some(String::from(item));
        }
        if let Some(item) = &self.avatar {
            avatar = Some(String::from(item));
        }
        if let Some(item) = &self.role_id {
            role_id = Some(String::from(item));
        }
        if let Some(item) = &self.department_id {
            department_id = Some(String::from(item));
        }
        if let Some(item) = &self.remark {
            remark = Some(String::from(item));
        }
        if let Some(item) = &self.is_admin {
            is_admin = Some(String::from(item));
        }

        core_models::UpdateUser {
            uid: String::from(&self.uid),
            username,
            nickname,
            password,
            status,
            email,
            gender,
            phone,
            avatar,
            role_id,
            department_id,
            remark,
            is_admin,
            last_login_ip: None,
            last_login_at: None,
            created_by: None,
            updated_by: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
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

///
/// 带分页的用户列表数据
///
#[derive(SimpleObject, Debug, Clone)]
pub struct ResUserList {
    pub data: Vec<Users>,
    pub page_info: crate::typings::PagerInfo,
}
