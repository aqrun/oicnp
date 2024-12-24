use clap::{Parser, Subcommand};

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
    Serve,
}