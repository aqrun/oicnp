#[macro_use]
extern crate lazy_static;

use cli::{Cli, Command};
use clap::{Parser};
use cmd::{
    run as blog_run, save_blogs,
    my_test::run as my_test_run,
    truncate_tables,
    init_user::run as init_user_run,
};
use oicnp_core::prelude::dotenv;
use oicnp_core::utils::get_env_config;


mod cli;
mod cmd;
mod models;
mod constants;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::FindAllBlogs { format } => {
            let blog_base = get_env_config("BLOG_BASE");
            blog_run(&format, blog_base.as_str(), &cli.dist_file).await;
        },
        Command::SaveBlogs => {
            save_blogs(&cli).await;
        },
        Command::MyTest => {
            my_test_run().await;
        },
        Command::TruncateTables => {
            truncate_tables().await;
        },
        Command::InitUser => {
            init_user_run().await;
        },
    }
}