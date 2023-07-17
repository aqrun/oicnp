use anyhow::{anyhow, Result};
use jsonwebtoken as jwt;
use chrono::Utc;
use crate::models::auth::{Claims};
use crate::G;
use log::error;

pub fn create_jwt(uid: &str, role: &str) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
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
        Ok(code) => Ok(code),
        Err(err) => {
            error!("Jwt 生成失败, {}, {:?}", uid, err);
            Ok(String::from(""))
        }
    }
}

pub fn decode_jwt(header_jwt: &str) -> Result<Claims> {
    let anonymous = Claims {
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
        &jwt::Validation::new(jwt::Algorithm::HS512),
    );

    match decoded {
        Ok(decoded) => Ok(decoded.claims),
        Err(err) => {
            Ok(anonymous)
        }
    }
}