use loco_rs::cli;
use oic::app::App;
use migration::Migrator;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenv::dotenv().ok();
    cli::main::<App, Migrator>().await
}
