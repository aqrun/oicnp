use cli::{Cli, Command};
use clap::{Parser};
use cmd::{
    run as blog_run, save_blogs,
    my_test::run as my_test_run,
};


mod cli;
mod cmd;
mod models;

#[tokio::main]
async fn main() {
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
        }
    }
}