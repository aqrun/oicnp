use crate::typings::OicRes;
use oicnp_core::prelude::anyhow::Result;

pub fn oic_ok<T>(data: T) -> Result<OicRes<T>> {
    let res = OicRes {
        code: String::from("200"),
        message: None,
        is_success: true,
        data: Some(data),
    };
    Ok(res)
}

pub fn oic_err<T>(code: &str, msg: String) -> Result<OicRes<T>> {
    let res = OicRes {
        code: String::from(code),
        message: Some(msg),
        is_success: false,
        data: None,
    };
    Ok(res)
}