use std::time::Duration;

use crate::G;
use sea_orm::{ConnectOptions, Database};
use tokio::sync::OnceCell;

pub use sea_orm::entity::prelude::DatabaseConnection;
pub use sea_orm::DbConn;

//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn establish_connection() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(G.database_url.to_owned());
    opt.max_connections(1000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt).await.expect("数据库打开失败");
    tracing::info!("Database connected");

    db
}