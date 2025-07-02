use captcha_rs::CaptchaBuilder;
use serde::Serialize;
use super::generate_uuid;

///
/// 验证码数据结构
/// 
#[derive(Debug, Clone, Serialize)]
pub struct AuthCaptcha {
    /// 验证码ID
    pub id: String,
    /// 验证码文本
    pub text: String,
    /// 验证码图片 base64 编码
    pub img: String,
}

impl AuthCaptcha {
    ///
    /// 接口合法的返回数据
    /// 
    pub fn data(&self) -> Self {
        Self {
            id: String::from(self.id.as_str()),
            text: String::from(""),
            img: String::from(self.img.as_str()),
        }
    }
}

///
/// 生成登陆验证码
/// 
pub fn get_auth_captcha() -> AuthCaptcha {
    let captcha = CaptchaBuilder::new()
		.length(4)
		.width(130)
		.height(32)
		.dark_mode(false)
		.complexity(1) // min: 1, max: 10
		.compression(40) // min: 1, max: 99
		.build();

    let text = String::from(captcha.text.as_str());
    let base64 = captcha.to_base64();
    let id = generate_uuid("captcha", 20);
    
    AuthCaptcha {
        id,
        text,
        img: base64,
    }
}
