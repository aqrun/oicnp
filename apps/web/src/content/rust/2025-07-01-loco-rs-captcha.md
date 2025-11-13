---
title: 'Rust Loco-rs 实现登陆验证码'
description: '验证码是防止机器人攻击和暴力破解的重要手段,本文将介绍如何使用 Rust 的 Loco-rs 框架实现一个完整的登录验证码功能，包括生成验证码图片、验证用户输入等功能'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', '验证码', 'loco-rs', 'captcha']
---

![img](https://cdn.oicnp.com/images/2025/login.png)

## 场景

在 Web 应用中，验证码是防止机器人攻击和暴力破解的重要手段。本文将介绍如何使用 Rust 的 Loco-rs 框架实现一个完整的登录验证码功能，包括生成验证码图片、验证用户输入等功能。

本文主要内容：

- 使用 `captcha-rs` 库生成验证码图片
- 在 Loco-rs 中实现验证码接口
- 使用缓存系统存储验证码数据
- 在前端集成验证码功能
- 在登录流程中验证用户输入的验证码

## 技术栈

- **Loco-rs**: Rust Web 框架，提供路由、中间件等功能
- **captcha-rs**: 验证码生成库，支持多种验证码样式和配置
- **serde**: 序列化/反序列化，用于 API 数据交换
- **axum**: HTTP 框架，Loco-rs 底层使用
- **Redis/Memory Cache**: 缓存系统，存储验证码数据

## 功能实现

### 1 生成验证码数据

首先，我们需要定义验证码的数据结构，并实现生成验证码的核心逻辑。

* 定义 `AuthCaptcha` 验证码结构体
* 实现方法 `get_auth_captcha` 生成登陆所需的验证码数据

```rust
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

```

### 2 定义 `/v1/captcha` 接口

接下来，我们需要创建一个 HTTP 接口来提供验证码服务。

实现接口 get `/v1/captcha`

生成验证码数据，并在缓存系统保存最新的验证码数据信息，过期时间10分钟。这样可以确保验证码的安全性，同时避免验证码永久存储在缓存中。

```rust
use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    utils::{get_auth_captcha, AuthCaptcha},
    typings::JsonRes,
    AppContext,
};
use std::time::Duration;

#[debug_handler]
pub async fn captcha(
    State(ctx): State<AppContext>,
) -> JsonRes<AuthCaptcha> {
    let captcha = get_auth_captcha();

    // 缓存验证码 10 分钟
    match ctx.cache.insert_with_expiry(
        captcha.id.as_str(),
        captcha.text.as_str(),
        Duration::from_secs(60 * 10),
    ).await {
        Ok(_) => {
            JsonRes::from((captcha.data(), "captcha"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

/// 注册路由
pub fn routes() -> Routes {
    Routes::new()
        .add("/captcha", get(captcha))
}
```

接口返回数据示例：

```json
{
    "code": "200",
    "data": {
        "captcha": {
            "id": "captcha-87lwww21jt5mtfio6581",
            "img": "data:image/jpeg;base64,/9j/4AAJGf/2Q=="
        }
    },
    "message": "success"
}
```

### 3 登陆界面添加验证表单

现在我们需要在前端界面中集成验证码功能。

添加验证码输入表单，并显示接口返回的 base64 格式验证码图片。这里使用 React 和 Ant Design 组件库来实现用户界面。

```jsx
<Form>
    {/* 邮箱密码省略 */}
    {/* 以下新增验证码表单 */}
    <div className="flex items-start mb-4">
        <Form.Item name="captcha" >
            <Input placeholder="验证码"/>
        </Form.Item>
        <div onClick={refreshCaptcha} >
            {/* 验证码图片 */}
            <Image
                src={captchaRes.img}
                alt="captcha"
                width={100}
                height={30}
            />
        </div>
    </div>
    {/* 登陆button */}
</Form>
```

### 4 登陆界面功能

前端需要实现验证码的交互逻辑，包括初始化和表单提交时的处理。

* 界面初始化时加载验证码数据
* 提交表单时补充验证码数据
* 验证码刷新功能，提升用户体验

```ts
const [captchaRes, setCaptchaRes] = useState<AuthCaptcha | null>(null);
const { fetchCaptcha } = useFetchCaptcha();
  
/**
 * 刷新验证码
 */
const refreshCaptcha = useMemoizedFn(async () => {
    const res = await fetchCaptcha();
    setCaptchaRes(res?.captcha);
});

/**
 * 表单提交操作
 */
const handleSubmit = useMemoizedFn(async () => {
    const values = form.getFieldsValue();

    const res = await loginAction({
        email: values?.email,
        password: values?.password,
        remember: Boolean(values?.remember),
        // 以下为验证码相关
        captchaId: captchaRes?.id,
        captcha: values?.captcha,
    });

    const code = res?.code || '200';

    if (code !== '200') {
        setErrorInfo(res?.message || '用户名或密码不正确');
        // 提交完成刷新一次验证码
        refreshCaptcha();
    }
});

/**
 * 界面初始数据加载
 */
useEffect(() => {
    refreshCaptcha();
}, []);
```

### 5 原有登陆接口补充验证码数据验证

最后，我们需要修改现有的登录接口，添加验证码验证逻辑。

* 从缓存系统获取对应的验证码明文
* 再对比用户输入和缓存信息
* 验证成功后删除缓存中的验证码，防止重复使用

```rust
/// 登陆接口定义
#[debug_handler]
async fn login(
    State(ctx): State<AppContext>,
    Json(params): Json<LoginParams>
) -> JsonRes<LoginResponse> {
    // 根据参数 captchaId 获取缓存的验证码文本
    let cache_captcha = match ctx.cache.get(params.captcha_id.as_str()).await {
        Ok(text) => text.unwrap_or(String::from("")),
        Err(_) => {
            return JsonRes::err("验证码已过期, 刷新后重试");
        }
    };

    let valid_cache = cache_captcha.to_lowercase();
    let valid_captcha = params.captcha.to_lowercase();
    
    // 忽略大小写对比用户输入的验证码
    if !valid_cache.eq(valid_captcha.as_str()) {
        return JsonRes::err("验证码错误");
    }

    // 验证成功后删除缓存中的验证码，防止重复使用
    let _ = ctx.cache.remove(params.captcha_id.as_str()).await;
    
    // 原有的登陆功能
    let res = services::auth::login(&ctx.db, &ctx.config, params).await;

    JsonRes::from(res)
}
```

## 安全考虑

在实现验证码功能时，需要注意以下几个安全要点：

1. **验证码有效期**: 设置合理的过期时间（如10分钟），避免验证码长期有效
2. **一次性使用**: 验证成功后立即删除缓存中的验证码，防止重复使用
3. **大小写不敏感**: 验证时忽略大小写，提升用户体验
4. **错误处理**: 验证码错误时给出明确提示，并刷新验证码
5. **防暴力破解**: 结合其他安全措施，如登录失败次数限制

## 性能优化

1. **缓存策略**: 使用 Redis 或内存缓存存储验证码，提高响应速度
2. **图片压缩**: 通过 `compression` 参数控制图片质量，减少传输数据量
3. **异步处理**: 使用异步操作处理缓存读写，避免阻塞

## 总结

本文主要说明在 Rust Loco-rs 框架中实现完整的登录验证码功能。通过以下步骤：

1. **后端实现**: 使用 `captcha-rs` 库生成验证码，通过 HTTP 接口提供服务
2. **缓存管理**: 使用缓存系统存储验证码数据，设置合理的过期时间
3. **前端集成**: 在登录界面添加验证码输入框和图片显示
4. **验证逻辑**: 在登录接口中添加验证码验证，确保安全性

初次实现验证码功能，主要考虑安全性、用户体验、易于维护、性能等方面，
是否可以有效防止机器人攻击和暴力破解，提升应用的安全性，还有待进一步验证。
任何改进或建议欢迎评论 ^_^。


