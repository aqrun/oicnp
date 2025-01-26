//! Axum middleware for validating token header
//!
//! # Example:
//!
//! ```
//! use loco_rs::prelude::*;
//! use serde::Serialize;
//! use axum::extract::State;
//!
//! #[derive(Serialize)]
//! pub struct TestResponse {
//!     pub pid: String,
//! }
//!
//! async fn current(
//!     auth: auth::JWT,
//!     State(ctx): State<AppContext>,
//! ) -> Result<Response> {
//!     format::json(TestResponse{ pid: auth.claims.pid})
//! }
//! ```
use std::collections::HashMap;

use axum::{
    extract::{FromRef, FromRequestParts, Query},
    http::{request::Parts, HeaderMap},
};
use axum_extra::extract::cookie;
use serde::{Deserialize, Serialize};

use loco_rs::{
  app::AppContext,
  config::JWT as JWTConfig,
  errors::Error,
  model::Authenticable,
  Result as LocoResult,
};
use crate::auth::{UserClaims, JWT as AuthJWT};

// ---------------------------------------
//
// JWT Auth extractor
//
// ---------------------------------------

// Define constants for token prefix and authorization header
const TOKEN_PREFIX: &str = "Bearer ";
const AUTH_HEADER: &str = "authorization";

// Define a struct to represent user authentication information serialized
// to/from JSON
#[derive(Debug, Deserialize, Serialize)]
pub struct JWTWithUser<T: Authenticable> {
    pub claims: UserClaims,
    pub user: T,
}

// Implement the FromRequestParts trait for the Auth struct
impl<S, T> FromRequestParts<S> for JWTWithUser<T>
where
    AppContext: FromRef<S>,
    S: Send + Sync,
    T: Authenticable + Default,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        let ctx: AppContext = AppContext::from_ref(state);

        let token = match extract_token(get_jwt_from_config(&ctx)?, parts) {
          Ok(token) => token,
          _ => {
            let empty_res = Self {
              claims: UserClaims {
                uid: "".to_string(),
                uuid: "".to_string(),
                exp: 0,
                claims: None,
              },
              user: T::default(),
            };
            return Ok(empty_res);
          },
        };

        println!("token: {}", token);
        let jwt_secret = ctx.config.get_jwt_config()?;

        match AuthJWT::new(&jwt_secret.secret).validate(&token) {
            Ok(claims) => {
                let user = T::find_by_claims_key(&ctx.db, &claims.claims.uid)
                    .await
                    .map_err(|_| Error::Unauthorized("token is not valid".to_string()))?;
                Ok(Self {
                    claims: claims.claims,
                    user,
                })
            }
            Err(_err) => {
                return Err(Error::Unauthorized("token is not valid".to_string()));
            }
        }
    }
}

// Define a struct to represent user authentication information serialized
// to/from JSON
#[derive(Debug, Deserialize, Serialize)]
pub struct JWT {
    pub claims: UserClaims,
}

// Implement the FromRequestParts trait for the Auth struct
impl<S> FromRequestParts<S> for JWT
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        let ctx: AppContext = AppContext::from_ref(state); // change to ctx

        let token = extract_token(get_jwt_from_config(&ctx)?, parts)?;

        let jwt_secret = ctx.config.get_jwt_config()?;

        match AuthJWT::new(&jwt_secret.secret).validate(&token) {
            Ok(claims) => Ok(Self {
                claims: claims.claims,
            }),
            Err(_err) => {
                return Err(Error::Unauthorized("token is not valid".to_string()));
            }
        }
    }
}

/// extract JWT token from context configuration
///
/// # Errors
/// Return an error when JWT token not configured
fn get_jwt_from_config(ctx: &AppContext) -> LocoResult<&JWTConfig> {
    ctx.config
        .auth
        .as_ref()
        .ok_or_else(|| Error::string("auth not configured"))?
        .jwt
        .as_ref()
        .ok_or_else(|| Error::string("JWT token not configured"))
}
/// extract token from the configured jwt location settings
fn extract_token(jwt_config: &JWTConfig, parts: &Parts) -> LocoResult<String> {
    #[allow(clippy::match_wildcard_for_single_variants)]
    match jwt_config
        .location
        .as_ref()
        .unwrap_or(&loco_rs::config::JWTLocation::Bearer)
    {
        loco_rs::config::JWTLocation::Query { name } => extract_token_from_query(name, parts),
        loco_rs::config::JWTLocation::Cookie { name } => extract_token_from_cookie(name, parts),
        loco_rs::config::JWTLocation::Bearer => extract_token_from_header(&parts.headers)
            .map_err(|e| Error::Unauthorized(e.to_string())),
    }
}
/// Function to extract a token from the authorization header
///
/// # Errors
///
/// When token is not valid or out found
pub fn extract_token_from_header(headers: &HeaderMap) -> LocoResult<String> {
    Ok(headers
        .get(AUTH_HEADER)
        .ok_or_else(|| Error::Unauthorized(format!("header {AUTH_HEADER} token not found")))?
        .to_str()
        .map_err(|err| Error::Unauthorized(err.to_string()))?
        .strip_prefix(TOKEN_PREFIX)
        .ok_or_else(|| Error::Unauthorized(format!("error strip {AUTH_HEADER} value")))?
        .to_string())
}

/// Extract a token value from cookie
///
/// # Errors
/// when token value from cookie is not found
pub fn extract_token_from_cookie(name: &str, parts: &Parts) -> LocoResult<String> {
    // LogoResult
    let jar: cookie::CookieJar = cookie::CookieJar::from_headers(&parts.headers);
    Ok(jar
        .get(name)
        .ok_or(Error::Unauthorized("token is not found".to_string()))?
        .to_string()
        .strip_prefix(&format!("{name}="))
        .ok_or_else(|| Error::Unauthorized("error strip value".to_string()))?
        .to_string())
}
/// Extract a token value from query
///
/// # Errors
/// when token value from cookie is not found
pub fn extract_token_from_query(name: &str, parts: &Parts) -> LocoResult<String> {
    // LogoResult
    let parameters: Query<HashMap<String, String>> =
        Query::try_from_uri(&parts.uri).map_err(|err| Error::Unauthorized(err.to_string()))?;
    parameters
        .get(name)
        .cloned()
        .ok_or_else(|| Error::Unauthorized(format!("`{name}` query parameter not found")))
}

// ---------------------------------------
//
// API Token Auth / Extractor
//
// ---------------------------------------
#[derive(Debug, Deserialize, Serialize)]
// Represents the data structure for the API token.
pub struct ApiToken<T: Authenticable> {
    pub user: T,
}

// Implementing the `FromRequestParts` trait for `ApiToken` to enable extracting
// it from the request.
impl<S, T> FromRequestParts<S> for ApiToken<T>
where
    AppContext: FromRef<S>,
    S: Send + Sync,
    T: Authenticable,
{
    type Rejection = Error;

    // Extracts `ApiToken` from the request parts.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        // Extract API key from the request header.
        let api_key = extract_token_from_header(&parts.headers)?;

        // Convert the state reference to the application context.
        let state: AppContext = AppContext::from_ref(state);

        // Retrieve user information based on the API key from the database.
        let user = T::find_by_api_key(&state.db, &api_key)
            .await
            .map_err(|e| Error::Unauthorized(e.to_string()))?;

        Ok(Self { user })
    }
}
