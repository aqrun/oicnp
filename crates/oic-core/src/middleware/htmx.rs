use axum::{
  extract::FromRequestParts,
  http::request::Parts,
};

pub struct HtmxRequest {
  pub is_htmx: bool,
  pub current_url: Option<String>,
  pub prompt: Option<String>,
  pub target: Option<String>,
  pub trigger: Option<String>,
}

impl<S> FromRequestParts<S> for HtmxRequest
where
  S: Send + Sync,
{
  type Rejection = ();

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
      let headers = &parts.headers;
      
      let is_htmx = headers
          .get("hx-request")
          .and_then(|v| v.to_str().ok())
          .map(|v| v == "true")
          .unwrap_or(false);
      
      let current_url = headers
          .get("hx-current-url")
          .and_then(|v| v.to_str().ok())
          .map(|s| s.to_string());
      
      let prompt = headers
          .get("hx-prompt")
          .and_then(|v| v.to_str().ok())
          .map(|s| s.to_string());
      
      let target = headers
          .get("hx-target")
          .and_then(|v| v.to_str().ok())
          .map(|s| s.to_string());
      
      let trigger = headers
          .get("hx-trigger")
          .and_then(|v| v.to_str().ok())
          .map(|s| s.to_string());
      
      Ok(HtmxRequest {
          is_htmx,
          current_url,
          prompt,
          target,
          trigger,
      })
  }
}