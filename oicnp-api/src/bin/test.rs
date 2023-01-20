extern crate fast_log;

use oicnp_api::typings::TaxonomyBundle;
use fast_log::{
    plugin::{
        file_split::RollingType,
        packer::LogPacker,
    },
    consts::LogSize,
};
use log::{info};

#[tokio::main]
async fn main() {
    fast_log::init(fast_log::Config::new()
        .console()
        .chan_len(Some(100000))
        .file_split(
            "target/logs/",
            LogSize::MB(1),
            RollingType::All,
            LogPacker{}
        )).unwrap();
    // fast_log::init_log("target/test.log",
    //                    // 1000,
    //                    log::Level::Warn,
    //                    None,
    //                    true);
    info!("Commencing yak shaving----");
    log::logger().flush();
}