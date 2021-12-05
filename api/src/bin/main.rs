use tokio::task;
use async_graphql::{Result};
use api::run;

#[tokio::main]
async fn main() -> Result<()> {
    let blocking_task = task::spawn_blocking(run());
    blocking_task.await?
}
