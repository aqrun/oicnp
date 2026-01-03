use axum::{
  response::{IntoResponse, Html, Response},
  http::StatusCode,
};
use askama::Template;

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