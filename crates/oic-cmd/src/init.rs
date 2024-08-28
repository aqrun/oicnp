use crate::{Cli, Command, cmd};
use clap::Parser;

pub async fn init_cmd() {
  dotenv::dotenv().ok();
  let cli = Cli::parse();

  match cli.command {
      // Command::FindAllBlogs { format } => {
      //     let blog_base = get_env_config("BLOG_BASE");
      //     blog_run(&format, blog_base.as_str(), &cli.dist_file).await;
      // },
      // Command::SaveBlogs => {
      //     save_blogs(&cli).await;
      // },
      // Command::TruncateTables => {
      //   truncate_tables().await;
      // },
      // Command::InitUser => {
      //   init_user_run().await;
      // },
      Command::MyTest => {
          cmd::my_test::execute().await;
      },
  }
}