use clap::{Parser, Subcommand};
use crate::cmd;
use oic_core::{
    app::{create_context, get_environment},
    utils::logger,
};
use loco_rs::environment::Environment;

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
    /// 密码加密 -p [pass]
    Hash {
        /// 指定明文密码
        #[clap(short = 'p', long, default_value = "")]
        password: String,
    },
    /// 收集所有博客数据
    FindAllBlogs {
        #[clap(default_value = "json")]
        format: String,
    },
    /// 保存博客数据到数据库
    SaveBlogs,
    MyTest,
    // TruncateTables,
    // InitUser,
    
    /// 数据表创建初始数据
    InitSeed,

    /// 初始化数据
    /// SeedData,

    /// 启动接口服务
    ///
    Serve,

    /// 诗词相关命令
    Poetry {
        #[clap(subcommand)]
        command: PoetryCommand,
    },
}

#[derive(Subcommand)]
pub enum PoetryCommand {
    /// 初始化诗词数据表
    Init,
    /// 读取诗词数据到数据库
    SyncData,
}

pub async fn init_cmd() {
    dotenv::dotenv().ok();
    let environment = get_environment();
    let app_ctx = create_context(&environment).await.expect("Context 创建失败");

    logger::init(&app_ctx.config.logger).expect("Logger 初始化失败");
    let task_span = create_root_span(&environment);
    let _guard = task_span.enter();

    let cli = Cli::parse();

    match cli.command {
        Command::FindAllBlogs { format } => {
            let blog_base = dotenv::var("BLOG_BASE").expect("BLOG_BASE 环境变量未设置");
            if let Err(err) = cmd::blog::find_all_blog(
                format.as_str(),
                blog_base.as_str(),
                &cli.dist_file
            ).await {
                log::error!("FindAllBlogs: {:?}", err);
            }
        },
        Command::SaveBlogs => {
            if let Err(err) = cmd::blog::save_blogs(
                &app_ctx,
                cli.dist_file.as_str()
            ).await {
                log::error!("SaveBlogs: {:?}", err);
            }
        },
        // Command::TruncateTables => {
        //   truncate_tables().await;
        // },
        // Command::InitUser => {
        //   init_user_run().await;
        // },
        Command::MyTest => {
            if let Err(err) = cmd::my_test::run(&app_ctx).await {
                log::error!("MyTest: {:?}", err);
            }
        },

        Command::Hash { password } => {
            if let Err(err) = cmd::hash_pass::hash(password.as_str()).await {
                log::error!("HashPassErr: {:?}", err);
            }
        },

        Command::InitSeed => {
            if let Err(err) = cmd::init_seed::run(&app_ctx).await {
                log::error!("InitSeedErr: {}", err);
            }
        },
        // Command::SeedData => {
        //     if let Err(err) = cmd::seed_data::run(&app_ctx).await {
        //         log::error!("SeedDataErr: {}", err);
        //     }
        // },
        Command::Serve => {
            let _ = oic_web::app::run().await;
        },

        Command::Poetry { command } => {
            match command {
                PoetryCommand::Init => {
                    if let Err(err) = cmd::poetry::init_poetry().await {
                        log::error!("InitPoetryErr: {}", err);
                    }
                },
                PoetryCommand::SyncData => {
                    if let Err(err) = cmd::poetry::sync_data().await {
                        log::error!("SyncDataErr: {}", err);
                    }
                },
            }
        },
    }
}

fn create_root_span(environment: &Environment) -> tracing::Span {
    tracing::span!(tracing::Level::DEBUG, "app", environment = %environment)
}
