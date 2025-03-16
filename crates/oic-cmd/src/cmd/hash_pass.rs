use oic_core::utils::{generate_salt, encrypt_password};
use anyhow::{Result, anyhow};

/**
 * 密码加密
 * 并生成 salt
 */
pub async fn hash(password: &str) -> Result<()> {
    if password.is_empty() {
        return Err(anyhow!("密码不能为空"));
    }

    let salt = generate_salt();
    let hash = encrypt_password(salt.as_str(), password);

    println!(r#"
password: {password}
salt: {salt}
hash: {hash}
"#);
    Ok(())
}
