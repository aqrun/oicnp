use poem::{handler, listener::TcpListener, get, web::{Path, Query, Redirect, Html}, Route, Server, Response, IntoResponse};
use serde::Deserialize;
use oicnp_core::prelude::{
    tracing_subscriber, tokio,
};
// use askama::Template;

// #[derive(Template)] // this will generate the code...
// #[template(source = "warning-page.html", ext = "html")] // using the template in this path, relative
// struct WarningPageTemplate<'a> {
//     target: &'a str,
// }

#[derive(Debug, Deserialize)]
struct WarningPageUrlParams {
    pub target: Option<String>,
}

#[handler]
fn short_link(Path(link_name): Path<String>) -> Redirect {
    let url = format!("/a?target={}", link_name);
    Redirect::moved_permanent(url)
}

#[handler]
fn warning_page(
    Query(WarningPageUrlParams { target }): Query<WarningPageUrlParams>
) -> Response {
    let target = target.unwrap_or("".to_string());
    // let tpl = WarningPageTemplate {
    //     target: target.as_str(),
    // };
    // tpl.render().unwrap_or("".to_string()).into_response()
    "abc".into_response()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/a/:link_name", get(short_link))
        .at("/a", get(warning_page))
        ;
    let listener = TcpListener::bind("127.0.0.1:3000");
    let server = Server::new(listener);
    server.run(app).await
}
