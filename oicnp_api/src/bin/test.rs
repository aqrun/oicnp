extern crate fast_log;

use oicnp_api::typings::TaxonomyBundle;

#[tokio::main]
async fn main() {
    fast_log::init_log("target/test.log",
                       // 1000,
                       log::Level::Warn,
                       None,
                       true);
    println!("-------{:?}", TaxonomyBundle::Category.to_string());
}