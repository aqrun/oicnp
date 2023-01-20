extern crate fast_log;
// use tokio::task;
use oicnp_api::run;

#[tokio::main]
async fn main() {
    // fast_log::init_log("target/api.log",
    //                    log::Level::Warn,
    //                    None,
    //                    false)
    //     .expect("Init log failed");
    // let blocking_task = task::spawn_blocking(run());
    // blocking_task.await?
    run().await.expect("Run start failed");
}
