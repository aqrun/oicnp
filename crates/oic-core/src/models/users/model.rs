use async_trait::async_trait;
use chrono::offset::Utc;
use loco_rs::{hash, prelude::*};
use uuid::Uuid;
use crate::utils::catch_err;
use crate::{
    auth::JWT,
    typings::ListData,
    uuid,
};
use super::{RegisterParams, Validator};
pub use crate::entities::prelude::{
  UserActiveModel,
  UserEntity,
  UserModel,
  UserColumn,
  RoleModel,
  UserRoleMapActiveModel,
  UserRoleMapEntity,
};
use crate::{RequestParamsUpdater, ModelCrudHandler};
use sea_orm::{prelude::*, QueryOrder};
use super::{
    UserFilters,
    CreateUserReqParams,
    UpdateUserReqParams,
    DeleteUserReqParams,
};

impl Validatable for UserActiveModel {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            name: self.username.as_ref().to_owned(),
            email: self.email.as_ref().to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for UserActiveModel {
    /// 
    /// before save
    ///  
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.validate()?;

        if insert {
            let mut this = self;

            if this.uuid.is_not_set() {
                this.uuid = ActiveValue::Set(uuid!());
            }

            if this.api_key.is_not_set() {
                this.api_key = ActiveValue::Set(format!("api-{}", Uuid::new_v4()));
            }

            return Ok(this);
        }

        Ok(self)
    }
}

#[async_trait]
impl Authenticable for UserModel {
    async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = UserEntity::find()
            .filter(
                model::query::condition()
                    .eq(UserColumn::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
        Self::find_by_uuid(db, claims_key).await
    }
}

#[async_trait]
impl ModelCrudHandler for UserModel {
    type CreateReqParams = CreateUserReqParams;

    async fn create_multi(
        db: &DatabaseConnection,
        params: &[Self::CreateReqParams],
    ) -> ModelResult<String> {
        for item in params {
            catch_err(item.validate())?;
        }

        let txn = db.begin().await?;
        let mut users: Vec<UserActiveModel> = Vec::new();

        for item in params.iter() {
            let mut user = UserActiveModel::new();
            item.update(&mut user);
            item.update_by_create(&mut user);

            users.push(user);
        }

        let _ = UserEntity::insert_many(users).exec(&txn).await?;

        txn.commit().await?;

        // 需要批量创建的 user role 关联关系
        let mut user_role_list: Vec<UserRoleMapActiveModel> = Vec::new();

        for item in params.iter() {
            let mut email = String::from("");
            let mut item_roles: Vec<String> = Vec::new();

            if let Some(x) = &item.email {
                email = String::from(x);
            } else {
                continue;
            }

            if let Some(x) = &item.roles {
                item_roles = x.clone();
            }

            let user = Self::find_by_email(db, email.as_str()).await?;

            for role_item in item_roles {
                let role = RoleModel::find_by_vid(db, role_item.as_str()).await?;
                let user_role_map = UserRoleMapActiveModel {
                    uid: Set(user.uid),
                    role_id: Set(role.role_id),
                    ..Default::default()
                };
                user_role_list.push(user_role_map);
            }
        }

        let _ = UserRoleMapEntity::insert_many(user_role_list).exec(db).await?;
        
        Ok(String::from("批量user添加完成"))
    }
}

impl UserModel {
    ////
    /// 获取user列表
    /// 
    pub async fn find_list(db: &DatabaseConnection, params: UserFilters) -> ModelResult<ListData<UserModel>> {
        let page = params.get_page();
        let page_size = params.get_page_size();
        let order = params.get_order();
        let order_by_str = params.get_order_by();

        let mut q = UserEntity::find();

        if let Some(x) = params.uid {
            if x > 0 {
                q = q.filter(UserColumn::Uid.eq(x));
            }
        }

        if let Some(x) = params.uuid {
            if !x.is_empty() {
                q = q.filter(UserColumn::Uuid.eq(x));
            }
        }

        let mut order_by = UserColumn::Uid;

        if order_by_str.eq("created_at") {
            order_by = UserColumn::CreatedAt;
        }

        // 获取全部数据条数
        let total = q.clone().count(db).await?;
        // 分页获取数据
        let pager = q.order_by(order_by, order)
            .paginate(db, page_size);
        let list = pager.fetch_page(page - 1).await?;

        let res = ListData {
            data: list,
            page,
            page_size,
            total,
        };

        Ok(res)
    }

    /// 创建 user
    pub async fn create(db: &DatabaseConnection, params: &CreateUserReqParams) -> ModelResult<Self> {
        catch_err(params.validate())?;

        let mut user = UserActiveModel::new();
        params.update(&mut user);
        params.update_by_create(&mut user);

        println!("test---{:?}", user.clone());
        let user = user.insert(db).await?;

        Ok(user)
    }

    /// 给用户指定角色
    pub async fn assign_roles(
        db: &DatabaseConnection,
        uid: i64,
        role_vids: &[String],
    ) -> ModelResult<i64> {
        // 需要批量创建的 user role 关联关系
        let mut user_roles: Vec<UserRoleMapActiveModel> = Vec::new();

        for vid in role_vids {
            let role = RoleModel::find_by_vid(db, vid.as_str()).await?;
            let user_role_map = UserRoleMapActiveModel {
                uid: Set(uid),
                role_id: Set(role.role_id),
                ..Default::default()
            };
            user_roles.push(user_role_map);
        }

        let _ = UserRoleMapEntity::insert_many(user_roles).exec(db).await?;

        Ok(0)
    }

    /// 更新数据
    pub async fn update(db: &DatabaseConnection, params: UpdateUserReqParams) -> ModelResult<i64> {
        catch_err(params.validate())?;
        let uid = params.uid.unwrap_or(0);

        if uid < 0 {
            return Err(ModelError::Message(format!("数据不存在,id: {}", uid)));
        }

        let mut user = Self::find_by_uid(db, uid)
            .await?
            .into_active_model();    
        params.update(&mut user);
    
        let item = user.update(db).await?;
        Ok(item.uid)
    }

    /// 删除数据
    pub async fn delete(db: &DatabaseConnection, params: DeleteUserReqParams) -> ModelResult<i64> {
        let uid = params.uid.unwrap_or(0);

        if uid <= 0 {
            return Err(ModelError::Message(format!("数据不存在, uid: {}", uid)));
        }

        let _res = UserEntity::delete_by_id(uid)
            .exec(db)
            .await?;

        Ok(uid)
    }

    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
        let user = UserEntity::find()
            .filter(
                model::query::condition()
                    .eq(UserColumn::Email, email)
                    .build(),
            )
            .one(db)
            .await?;

        user.ok_or_else(|| ModelError::Message(format!("User not found")))
    }

    /// finds a user by the provided verification token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_verification_token(
        db: &DatabaseConnection,
        token: &str,
    ) -> ModelResult<Self> {
        let user = UserEntity::find()
            .filter(
                model::query::condition()
                    .eq(UserColumn::EmailVerifyToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided reset token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_reset_token(db: &DatabaseConnection, token: &str) -> ModelResult<Self> {
        let user = UserEntity::find()
            .filter(
                model::query::condition()
                    .eq(UserColumn::ResetToken, token)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided uid
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_uid(db: &DatabaseConnection, uid: i64) -> ModelResult<Self> {
        // let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let user = UserEntity::find()
            .filter(
                model::query::condition()
                    .eq(UserColumn::Uid, uid)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided uuid
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_uuid(db: &DatabaseConnection, uuid: &str) -> ModelResult<Self> {
        // let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
        let user = UserEntity::find()
            .filter(
                model::query::condition()
                    .eq(UserColumn::Uuid, uuid)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// finds a user by the provided api key
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_api_key(db: &DatabaseConnection, api_key: &str) -> ModelResult<Self> {
        let user = UserEntity::find()
            .filter(
                model::query::condition()
                    .eq(UserColumn::ApiKey, api_key)
                    .build(),
            )
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    ///
    /// # Errors
    ///
    /// when could not verify password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Asynchronously creates a user with a password and saves it to the
    /// database.
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    pub async fn create_with_password(
        db: &DatabaseConnection,
        params: &RegisterParams,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        if UserEntity::find()
            .filter(
                model::query::condition()
                    .eq(UserColumn::Email, &params.email)
                    .build(),
            )
            .one(&txn)
            .await?
            .is_some()
        {
            return Err(ModelError::EntityAlreadyExists {});
        }

        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        let user = UserActiveModel {
            email: ActiveValue::set(params.email.to_string()),
            password: ActiveValue::set(password_hash),
            username: ActiveValue::set(params.username.to_string()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(user)
    }

    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_jwt(&self, secret: &str, expiration: &u64) -> ModelResult<String> {
        Ok(JWT::new(secret).generate_token(expiration, self.uid, self.uuid.as_str(), None)?)
    }
}

impl UserActiveModel {
    /// Sets the email verification information for the user and
    /// updates it in the database.
    ///
    /// This method is used to record the timestamp when the email verification
    /// was sent and generate a unique verification token for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_email_verification_sent(
        mut self,
        db: &DatabaseConnection,
    ) -> ModelResult<UserModel> {
        self.email_verify_sent_at = ActiveValue::set(Some(Utc::now().naive_utc()));
        self.email_verify_token = ActiveValue::Set(Uuid::new_v4().to_string());
        Ok(self.update(db).await?)
    }

    /// Sets the information for a reset password request,
    /// generates a unique reset password token, and updates it in the
    /// database.
    ///
    /// This method records the timestamp when the reset password token is sent
    /// and generates a unique token for the user.
    ///
    /// # Arguments
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_forgot_password_sent(mut self, db: &DatabaseConnection) -> ModelResult<UserModel> {
        self.reset_sent_at = ActiveValue::set(Some(Utc::now().naive_utc()));
        self.reset_token = ActiveValue::Set(Uuid::new_v4().to_string());
        Ok(self.update(db).await?)
    }

    /// Records the verification time when a user verifies their
    /// email and updates it in the database.
    ///
    /// This method sets the timestamp when the user successfully verifies their
    /// email.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn verified(mut self, db: &DatabaseConnection) -> ModelResult<UserModel> {
        self.email_verified_at = ActiveValue::set(Some(Utc::now().naive_utc()));
        Ok(self.update(db).await?)
    }

    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.    
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(
        mut self,
        db: &DatabaseConnection,
        password: &str,
    ) -> ModelResult<UserModel> {
        self.password =
            ActiveValue::set(hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?);
        self.reset_token = ActiveValue::Set(String::from(""));
        self.reset_sent_at = ActiveValue::Set(None);
        Ok(self.update(db).await?)
    }
}
