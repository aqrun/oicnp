use loco_rs::prelude::*;
use loco_rs::config::Config;
use serde::{Deserialize, Serialize};
use crate::entities::prelude::*;
use crate::models::users::{LoginParams, RegisterParams};
use serde_json::{json, Value};
use anyhow::{Result, anyhow};
use crate::utils::{verify_password, catch_err};

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub uid: i64,
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &UserModel, token: &str) -> Self {
        Self {
            token: String::from(token),
            uid: user.uid,
            uuid: String::from(user.uuid.as_str()),
            username: String::from(user.username.as_str()),
            email: String::from(user.email.as_str()),
            is_verified: user.email_verified_at.is_some(),
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
    config: &Config,
    params: LoginParams,
) -> Result<LoginResponse> {
    catch_err(params.validate())?;

    let user = UserModel::find_by_email(db, params.email.as_str()).await?;

    // let valid = user.verify_password(&params.password);
    let valid = verify_password(
        params.password.as_str(), 
        user.password.as_str(),
        user.salt.as_str()
    )?;

    if !valid {
        return Err(anyhow!("invalid password"));
    }

    let jwt_secret = config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    Ok(LoginResponse::new(&user, token.as_str()))
}
