use clap::{Parser, Subcommand};
use crate::cmd;
use oic_core::app::{create_context, get_environment};

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
    /// 密码加密
    Hash {
        /// 指定明文密码
        #[clap(short = 'p', long, default_value = "")]
        password: String,
    },
    // FindAllBlogs {
    //     #[clap(default_value = "json")]
    //     format: String,
    // },
    // SaveBlogs,
    MyTest,
    // TruncateTables,
    // InitUser,
    
    /// 数据表创建初始数据
    InitSeed,

    /// 初始化数据
    SeedData,

    /// 启动接口服务
    ///
    Serve,
}

pub async fn init_cmd() {
    dotenv::dotenv().ok();
    let environment = get_environment();
    let app_ctx = create_context(&environment).await.expect("Context 创建失败");

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
            if let Err(err) = cmd::my_test::run(&app_ctx).await {
                println!("MyTest: {:?}", err);
            }
        },

        Command::Hash { password } => {
            if let Err(err) = cmd::hash_pass::hash(password.as_str()).await {
                println!("HashPassErr: {:?}", err);
            }
        },

        Command::InitSeed => {
            if let Err(err) = cmd::init_seed::run(&app_ctx).await {
                println!("InitSeedErr: {}", err);
            }
        },
        Command::SeedData => {
            if let Err(err) = cmd::seed_data::run(&app_ctx).await {
                println!("SeedDataErr: {}", err);
            }
        },
        Command::Serve => {
            let _ = oic_web::app::run().await;
        },
    }
}
