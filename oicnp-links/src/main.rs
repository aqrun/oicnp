use poem::{handler, listener::TcpListener, get, web::{Path, Query, Redirect, Html}, Route, Server, Response, IntoResponse};
use serde::Deserialize;
use oicnp_core::prelude::{
    tracing_subscriber, tokio, dotenv,
};
use askama::Template;
use oicnp_core::{
    establish_connection, DB,
    services::{find_short_link_by_id, update_short_link_viewed},
};

#[derive(Template)]
#[template(path = "warning-page.html")]
struct WarningPageTemplate<'a> {
    target: &'a str,
}

#[derive(Debug, Deserialize)]
struct WarningPageUrlParams {
    pub target: Option<String>,
    pub key: Option<String>,
}

#[handler]
async fn short_link(Path(short_link_id): Path<String>) -> Redirect {
    let db = DB.get_or_init(establish_connection).await;
    let mut target = String::new();

    if let Ok(res) = find_short_link_by_id(db, &short_link_id).await {
        target = String::from(&res.link);
    }

    let url = format!("/a?key={}&target={}", short_link_id, target);
    Redirect::temporary(url)
}

#[handler]
async fn warning_page(
    Query(WarningPageUrlParams { target, key }): Query<WarningPageUrlParams>
) -> Response {
    if let Some(key) = key {
        let db = DB.get_or_init(establish_connection).await;
        update_short_link_viewed(db, &key).await.unwrap();
    }

    let target = target.unwrap_or("".to_string());
    let tpl = WarningPageTemplate {
        target: target.as_str(),
    };
    let dom = tpl.render().unwrap_or("".to_string());
    Html(dom).into_response()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/a/:short_link_id", get(short_link))
        .at("/a", get(warning_page))
        ;
    let listener = TcpListener::bind("0.0.0.0:8199");
    let server = Server::new(listener);
    server.run(app).await
}
