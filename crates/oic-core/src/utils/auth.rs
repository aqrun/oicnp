use ring::{digest, pbkdf2};
use std::num::NonZeroU32;
use anyhow::{anyhow, Result};
use data_encoding::HEXLOWER;
use rand::Rng;
use once_cell::sync::Lazy;
use std::sync::Arc;

// 加密算法
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
// 算法输出长度
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
// 算法迭代次数
static PBKDF2_ITERATIONS: Lazy<Arc<NonZeroU32>> = Lazy::new(|| {
    Arc::new(NonZeroU32::new(100_000).unwrap())
});

///
/// 生成密码随机salt 64位字符串
///
pub fn generate_salt() -> String {
    // 32位u8数组
    let mut salt = [0u8; CREDENTIAL_LEN];
    // 填充随机数
    rand::thread_rng().fill(&mut salt[..]);
    // 转为16进制字符串
    HEXLOWER.encode(&salt)
}

///
/// 密码序列化
///
pub fn encrypt_password(salt: &str, password: &str) -> String {
    let pbkdf2_iterations = PBKDF2_ITERATIONS.clone();
    let mut to_store: [u8; CREDENTIAL_LEN] = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        *pbkdf2_iterations,
        salt.as_bytes(),
        password.as_bytes(),
        &mut to_store
    );
    // 将类型[u8; CREDENTIAL_LEN] 转为 字符串
    HEXLOWER.encode(&to_store)
}

///
/// 较验密码
///
pub fn verify_password(password: &str, actual_password: &str, salt: &str) -> Result<bool> {
    // 将存储的字符串密码解析为u8数组
    let mut actual_password_decode: Vec<u8> = Vec::new();

    if let Ok(res) = HEXLOWER.decode(actual_password.as_bytes()) {
        actual_password_decode = res;
    }

    let pbkdf2_iterations = PBKDF2_ITERATIONS.clone();

    // 较验密码是否匹配
    match pbkdf2::verify(
        PBKDF2_ALG,
        *pbkdf2_iterations,
        salt.as_bytes(),
        password.as_bytes(),
        actual_password_decode.as_slice()
    ) {
        Ok(_) => Ok(true),
        _=> {
            Err(anyhow!("Failed"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::{generate_salt, encrypt_password, verify_password};

    /// 生成64位随机salt
    #[test]
    fn test_generate_salt() {
        let salt = generate_salt();
        assert_eq!(salt.len(), 64);
    }

    /// 测试密码加密
    #[test]
    fn test_encrypt_password() {
        let salt = "030c8d02eea6e5e5219096bd076c41e58e955632d59beb7d44fa18e3fbccb0bd";
        let password = "abc123";
        let res = encrypt_password(salt, password);
        assert_eq!(res, "76b0f1985d149b9d3770b3ef3c8d59b5ec9ec32ad0499e0882a25c567ccf99d6");
    }

    /// 测试密码验证正确
    #[test]
    fn test_password_verify() {
        let salt = "030c8d02eea6e5e5219096bd076c41e58e955632d59beb7d44fa18e3fbccb0bd";
        let actual_pass = "76b0f1985d149b9d3770b3ef3c8d59b5ec9ec32ad0499e0882a25c567ccf99d6";
        let password = "abc123";
        let res = verify_password(password, actual_pass, salt);
        assert!(res.is_ok());
    }

    /// 测试密码验证不正确
    #[test]
    fn test_password_verify_false() {
        let salt = "030c8d02eea6e5e5219096bd076c41e58e955632d59beb7d44fa18e3fbccb0bd";
        let actual_pass = "76b0f1985d149b9d3770b3ef3c8d59b5ec9ec32ad0499e0882a25c567ccf99d6";
        let password = "abc1231";
        let res = verify_password(password, actual_pass, salt);
        assert!(res.is_err());
    }
}