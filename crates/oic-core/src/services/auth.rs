use std::sync::Arc;
use loco_rs::prelude::*;
use loco_rs::config::Config;
use serde::{Deserialize, Serialize};
use crate::entities::prelude::*;
use crate::models::users::{LoginParams, RegisterParams};
use serde_json::{json, Value};
use anyhow::{Result, anyhow};
use crate::utils::{verify_password, catch_err};
use crate::services::cache::OicCache;
use std::time::Duration;
use crate::uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenParams {
    #[serde(rename(deserialize = "refreshToken"))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub uid: i64,
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub is_verified: bool,
    pub remember: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &UserModel, token: &str, refresh_token: &str) -> Self {
        Self {
            token: String::from(token),
            refresh_token: String::from(refresh_token),
            uid: user.uid,
            uuid: String::from(user.uuid.as_str()),
            username: String::from(user.username.as_str()),
            email: String::from(user.email.as_str()),
            is_verified: user.email_verified_at.is_some(),
            remember: false,
        }
    }
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
pub async fn register(
    db: &DatabaseConnection,
    params: RegisterParams,
) -> Result<Value> {
    let res = UserModel::create_with_password(db, &params).await;

    let user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            return Err(anyhow!("could not register user"));
        }
    };

    let user = user
        .into_active_model()
        .set_email_verification_sent(db)
        .await?;

    // 不发送邮件
    // AuthMailer::send_welcome(&ctx, &user).await?;

    Ok(json!({
        "uid": user.uid.to_string(),
        "uuid": user.uuid.to_string(),
        "username": user.username,
        "email": user.email,
    }))
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
pub async fn verify(
    db: &DatabaseConnection,
    params: VerifyParams,
) -> Result<()> {
    let user = UserModel::find_by_verification_token(db, &params.token).await?;

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.uuid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(db).await?;
        tracing::info!(pid = user.uuid.to_string(), "user verified");
    }

    Ok(())
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
pub async fn forgot(
    db: &DatabaseConnection,
    params: ForgotParams,
) -> Result<()> {
    let Ok(user) = UserModel::find_by_email(db, params.email.as_str()).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return Err(anyhow!("email user not found"));
    };

    let _user = user
        .into_active_model()
        .set_forgot_password_sent(db)
        .await?;

    // AuthMailer::forgot_password(&ctx, &user).await?;

    Ok(())
}

/// reset user password by the given parameters
pub async fn reset(db: &DatabaseConnection, params: ResetParams) -> Result<()> {
    let Ok(user) = UserModel::find_by_reset_token(db, params.token.as_str()).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return Err(anyhow!("reset token not found"));
    };
    user.into_active_model()
        .reset_password(db, params.password.as_str())
        .await?;

    Ok(())
}

/// Creates a user login and returns a token
pub async fn login(
    db: &DatabaseConnection,
    params: LoginParams,
) -> Result<LoginResponse> {
    if params.captcha.is_empty() {
        return Err(anyhow!("验证码不能为空"));
    }

    catch_err(params.validate())?;

    let user = UserModel::find_by_email(db, params.email.as_str()).await?;

    // let valid = user.verify_password(&params.password);
    let valid = match verify_password(
        params.password.as_str(), 
        user.password.as_str(),
        user.salt.as_str()
    ) {
        Ok(valid) => valid,
        Err(err) => {
            tracing::info!(message = err.to_string(), "invalid password");
            false
        }
    };

    if !valid {
        return Err(anyhow!("用户名或密码错误"));
    }

    let session_id = uuid!();
    let refresh_token = uuid!();
    let mut login_res = LoginResponse::new(&user, session_id.as_str(), refresh_token.as_str());
    login_res.remember = params.remember;

    Ok(login_res)
}

/// Creates a user login and returns a token
/// 
/// * Access Token: 短期有效（15分钟-1小时）
/// * Refresh Token: 长期有效（7-30天）
/// 
/// 增加登陆尝试次数限制
/// 
pub async fn access_token(
    db: &DatabaseConnection,
    cache: Arc<OicCache>,
    config: &Config,
    params: LoginParams,
) -> Result<LoginResponse> {
    catch_err(params.validate())?;

    let user = UserModel::find_by_email(db, params.email.as_str()).await?;

    // let valid = user.verify_password(&params.password);
    let valid = match verify_password(
        params.password.as_str(), 
        user.password.as_str(),
        user.salt.as_str()
    ) {
        Ok(valid) => valid,
        Err(err) => {
            tracing::info!(message = err.to_string(), "invalid password");
            false
        }
    };

    if !valid {
        return Err(anyhow!("用户名或密码错误"));
    }

    let jwt_secret = config.get_jwt_config()?;
    let refresh_token = uuid!();

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    let cache_key = format!("api:refresh-token:{}", refresh_token);
    let duration = Duration::from_secs(30 * 24 * 3600);
    
    if let Err(err) = cache.insert_with_expiry(cache_key.as_str(), user.uuid.as_str(), duration).await {
        tracing::error!(message = err.to_string(), "failed to insert refresh token into cache");
        return Err(anyhow!("failed to insert refresh token into cache"));
    }
    Ok(LoginResponse::new(&user, token.as_str(), refresh_token.as_str()))
}

/// 使用 refresh token 获取新的 access token
/// 
/// * Access Token: 短期有效（15分钟-1小时）
/// * Refresh Token: 长期有效（7-30天）
/// 
/// 增加登陆尝试次数限制
/// 
pub async fn refresh_token(
    db: &DatabaseConnection,
    cache: Arc<OicCache>,
    config: &Config,
    params: &RefreshTokenParams,
) -> Result<LoginResponse> {
    let cache_key = format!("api:refresh-token:{}", params.refresh_token.as_str());
    let user_uuid = match cache.get(cache_key.as_str()).await {
        Ok(text) => text.unwrap_or(String::from("")),
        Err(_) => {
            return Err(anyhow!("Refresh token not found"));
        },
    };
    cache.remove(cache_key.as_str()).await?;

    if user_uuid.is_empty() {
        return Err(anyhow!("Refresh token not found"));
    }

    let user = UserModel::find_by_uuid(db, user_uuid.as_str()).await?;

    let jwt_data = config.get_jwt_config()?;
    let token = user
        .generate_jwt(&jwt_data.secret, &jwt_data.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    let refresh_token = uuid!();
    let cache_key = format!("api:refresh-token:{}", refresh_token);
    let duration = Duration::from_secs(30 * 24 * 3600);
    
    if let Err(err) = cache.insert_with_expiry(cache_key.as_str(), user.uuid.as_str(), duration).await {
        tracing::error!(message = err.to_string(), "failed to insert refresh token into cache");
        return Err(anyhow!("failed to insert refresh token into cache"));
    }
    Ok(LoginResponse::new(&user, token.as_str(), refresh_token.as_str()))
}