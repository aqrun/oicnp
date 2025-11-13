---
title: 'Rust 实现用户密码加密及验证'
description: '使用 ring::pbkdf2的密钥派生函数 pbkdf2::derive 来生成加盐密码的哈稀。 使用pbkdf2::verify验证哈希是否正确'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', 'encryption', '密码', '加密']
---

> 参考 [Rust Cookbook - Encryption](https://rust-lang-nursery.github.io/rust-cookbook/cryptography/encryption.html)

使用 [ring::pbkdf2](https://briansmith.org/rustdoc/ring/pbkdf2/index.html)的密钥派生函数 [pbkdf2::derive](https://briansmith.org/rustdoc/ring/pbkdf2/fn.derive.html) 来生成加盐密码的哈稀。 使用[pbkdf2::verify](https://briansmith.org/rustdoc/ring/pbkdf2/fn.verify.html)验证哈希是否正确。 随机盐使用 rand::thread_rng 生成随机字节数组，再转为 64 位字符串。

## 用到的库

- ring pbkdf2 加密算法库
- std::num::NonZeroU32 指定算法加密迭代次数
- data_encoding::HEXLOWER 字节数组和 16 进制字符串互转
- rand::Rng 随机数生成
- once_cell::sync::Lazy 配合 Arc 简化全局数据延迟生成
- anyhow 简化错误处理

## 代码实现

```rust
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
    let str_salt = HEXLOWER.encode(&salt);

    str_salt
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
    let str_result = HEXLOWER.encode(&to_store);

    str_result
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
```

## 单元测试

```rust
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
```

测试结果输出类似：

```txt
running 4 tests

test utils::auth::test::test_generate_salt ... ok
test utils::auth::test::test_encrypt_password ... ok
test utils::auth::test::test_password_verify_false ... ok
test utils::auth::test::test_password_verify ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s
```
