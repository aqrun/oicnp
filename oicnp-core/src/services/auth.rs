use crate::models::auth::{Claims, LoginInfo};
use crate::G;
use anyhow::{anyhow, Result};
use chrono::Utc;
use jsonwebtoken as jwt;
use log::error;

pub fn create_jwt(uid: &str, role: &str) -> Result<LoginInfo> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let header = jwt::Header::new(jwt::Algorithm::HS256);
    let claims = Claims {
        uid: String::from(uid),
        role: String::from(role),
        exp: expiration as usize,
    };
    let key = jwt::EncodingKey::from_secret(G.jwt_secret.as_bytes());

    let code = jwt::encode(&header, &claims, &key);

    match code {
        Ok(code) => Ok(LoginInfo {
            token: code,
            uid: claims.uid,
            role: claims.role,
            exp: claims.exp,
        }),
        Err(err) => {
            error!("Jwt 生成失败, {}, {:?}", uid, err);
            Err(anyhow!("Create failed"))
        }
    }
}

pub fn decode_jwt(header_jwt: &str, log_error: bool) -> Result<LoginInfo> {
    let anonymous = LoginInfo {
        token: String::from(header_jwt),
        uid: String::from(""),
        role: String::from("Anonymous"),
        exp: 0,
    };

    if header_jwt.eq("") {
        return Ok(anonymous);
    }

    let decoded = jwt::decode::<Claims>(
        &header_jwt,
        &jwt::DecodingKey::from_secret(G.jwt_secret.as_bytes()),
        &jwt::Validation::new(jwt::Algorithm::HS256),
    );

    match decoded {
        Ok(decoded) => {
            let info = LoginInfo {
                token: String::from(header_jwt),
                uid: decoded.claims.uid,
                role: decoded.claims.role,
                exp: decoded.claims.exp,
            };
            Ok(info)
        },
        Err(err) => {
            if log_error {
                error!("JWT 解析失败: {:}", err);
            }
            Ok(anonymous)
        }
    }
}
