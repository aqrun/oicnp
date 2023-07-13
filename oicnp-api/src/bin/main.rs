use oicnp_api::run;
use oicnp_core::prelude::dotenv;
use oicnp_core::prelude::tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    run().await.expect("Run start failed");
}
