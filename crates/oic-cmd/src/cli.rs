use clap::{Parser, Subcommand};
use crate::cmd;

#[derive(Parser)]
#[clap(version, author, about)]
pub struct Cli {
    #[clap(short = 'c', long, default_value = "config.toml")]
    pub config: Option<String>,

    #[clap(default_value = "target/blogs.json")]
    pub dist_file: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    // FindAllBlogs {
    //     #[clap(default_value = "json")]
    //     format: String,
    // },
    // SaveBlogs,
    MyTest,
    // TruncateTables,
    // InitUser,

    /// 初始化数据
    SeedData,

    /// 启动接口服务
    ///
    Serve,
}

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

        Command::SeedData => {
            if let Err(err) = cmd::seed_data::run().await {
                println!("SeedDataErr: {}", err);
            }
        },
        Command::Serve => {
            let _ = oic_web::app::run().await;
        },
    }
}
