use oicnp_core::prelude::{
    fast_log, tokio,
};
// use tokio::task;
use oicnp_api::run;
use oicnp_core::prelude::dotenv;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // fast_log::init_log("target/api.log",
    //                    log::Level::Warn,
    //                    None,
    //                    false)
    //     .expect("Init log failed");
    // let blocking_task = task::spawn_blocking(run());
    // blocking_task.await?
    run().await.expect("Run start failed");
}
