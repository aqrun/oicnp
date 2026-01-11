use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use crate::models::HtmlTemplate;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    #[allow(dead_code)]
    pub status_code: u16,
    pub title: String,
    pub message: String,
}

impl ErrorTemplate {
    pub fn new(status_code: StatusCode) -> Self {
        let (title, message) = match status_code {
            StatusCode::NOT_FOUND => (
                "页面未找到".to_string(),
                "抱歉，您访问的页面不存在。".to_string(),
            ),
            StatusCode::INTERNAL_SERVER_ERROR => (
                "服务器错误".to_string(),
                "我们正在努力修复这个问题，请稍后再试。".to_string(),
            ),
            _ => (
                "页面加载失败".to_string(),
                "抱歉，页面加载失败，请稍后再试。".to_string(),
            ),
        };

        Self {
            status_code: status_code.as_u16(),
            title,
            message,
        }
    }
}

/// 渲染错误页面
pub async fn render_error_page(
    status_code: StatusCode,
) -> anyhow::Result<HtmlTemplate<ErrorTemplate>> {
    let template = ErrorTemplate::new(status_code);
    Ok(HtmlTemplate(template))
}

/// 降级错误响应（当错误页面模板也渲染失败时使用）
pub fn fallback_error_response(status_code: StatusCode) -> impl IntoResponse {
    let (title, message) = match status_code {
        StatusCode::NOT_FOUND => ("页面未找到", "抱歉，您访问的页面不存在。"),
        StatusCode::INTERNAL_SERVER_ERROR => ("服务器错误", "我们正在努力修复这个问题，请稍后再试。"),
        _ => ("页面加载失败", "抱歉，页面加载失败，请稍后再试。"),
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 600px;
            margin: 100px auto;
            text-align: center;
            padding: 20px;
            color: #333;
        }}
        h1 {{ font-size: 48px; margin-bottom: 20px; }}
        h2 {{ font-size: 24px; margin-bottom: 16px; }}
        p {{ font-size: 16px; color: #666; margin-bottom: 32px; line-height: 1.6; }}
        a {{
            display: inline-block;
            padding: 12px 24px;
            background: #007bff;
            color: white;
            text-decoration: none;
            border-radius: 4px;
        }}
    </style>
</head>
<body>
    <h1>😔</h1>
    <h2>{}</h2>
    <p>{}</p>
    <a href="/">返回首页</a>
</body>
</html>"#,
        title, title, message
    );

    (status_code, Html(html))
}

/// 统一的错误处理函数
pub async fn handle_error(status_code: StatusCode, error: anyhow::Error) -> Response {
    eprintln!("Application error ({}): {}", status_code, error);
    
    match render_error_page(status_code).await {
        Ok(error_template) => {
            match error_template.0.render() {
                Ok(html) => (status_code, Html(html)).into_response(),
                Err(_) => fallback_error_response(status_code).into_response(),
            }
        }
        Err(_) => fallback_error_response(status_code).into_response(),
    }
}
