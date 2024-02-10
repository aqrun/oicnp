use oicnp_api::run;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    run().await.expect("Run start failed");
}
