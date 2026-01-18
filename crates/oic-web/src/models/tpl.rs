use axum::{
  response::{IntoResponse, Html, Response},
  http::StatusCode,
};
use askama::Template;
use bytes::Bytes;
use anyhow::Result;

pub struct HtmlTemplate<T> (pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where T: Template
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

/// Trait for rendering Askama templates directly to Bytes
/// 
/// This trait provides a convenient method to render templates directly to bytes,
/// avoiding the intermediate String allocation that would occur with `render()`.
pub trait RenderBytes: Template {
    /// Render the template directly to Bytes
    /// 
    /// This method uses `write_into` to write directly to a Vec<u8> buffer,
    /// which is then converted to Bytes, avoiding the String intermediate step.
    fn render_bytes(&self) -> Result<Bytes> {
        let mut buffer = Vec::new();
        Template::write_into(self, &mut buffer)
            .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?;
        Ok(Bytes::from(buffer))
    }
}

// Blanket implementation: all types that implement Template also implement RenderBytes
impl<T: Template> RenderBytes for T {}
