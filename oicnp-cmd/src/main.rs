#[macro_use]
extern crate lazy_static;

use cli::{Cli, Command};
use clap::{Parser};
use cmd::{
    run as blog_run, save_blogs,
    my_test::run as my_test_run,
    truncate_tables,
};
use oicnp_core::prelude::dotenv;


mod cli;
mod cmd;
mod models;
mod constants;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::FindAllBlogs { format, blog_base } => {
            blog_run(&format, &blog_base, &cli.dist_file).await;
        },
        Command::SaveBlogs => {
            save_blogs(&cli).await;
        },
        Command::TestRun => {
            my_test_run().await;
        },
        Command::TruncateTables => {
            truncate_tables().await;
        }
    }
}