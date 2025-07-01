use captcha_rs::CaptchaBuilder;
use serde::Serialize;
use super::generate_uuid;

#[derive(Debug, Clone, Serialize)]
pub struct AuthCaptcha {
    pub id: String,
    pub text: String,
    pub img: String,
}

impl AuthCaptcha {
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
		.height(40)
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
