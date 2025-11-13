---
title: 'Rust Tracing 日志同时输出到文件和控制台'
description: '通常日志会需要同时输出到文件或标准控制台, 输出到文件相当于持久存储，输出到控制台方便开发过程中错误信息查看。官方库示例好像不能正常运行'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'tracing', 'log', '日志']
---

[`tracing`](https://github.com/tokio-rs/tracing) 是 **tokio** 团队出品的日志处理库。

`tracing` `tracing_subscriber` 有改动，官方库示例好像也不能正常运行。

通常日志会需要同时输出到文件或标准控制台。

输出到文件相当于持久存储，输出到控制台方便开发过程中错误信息查看。

多目标输出具体参考官方示例： [fmt-multiple-writer](https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-multiple-writers.rs)

这个代码会报错，但大体写法改动不多，通过查找 doc.rs 很容易修复。

## 用到的库

```rust
// Cargo.toml

[dependencies]
// Rust团队官方日志门面库
log = "^0"
// 提供 warn! error! info! 相关标准错误输出宏
tracing = "^0"
// 提供不同输出层初始化，以及生成注册中心绑定多个层
tracing-subscriber = { version = "^0.3", features = ["fmt", "std"]}
// 提供非阻塞的方式输出日志，如写入文件
tracing-appender = "^0"
// log 库 与 tracing 库适配层
tracing-log = "^0"
// 提供指定输出内容的颜色
colored = "^2"
```

## 代码示例

```rust
use colored::Colorize;
use tracing_subscriber::{fmt, filter::LevelFilter};
use tracing_subscriber::prelude::*;
use tracing_log::LogTracer;

fn main() {
  // 日志配置初始化
  // 这里需要接收 guard 否则文件输出内容会是空的
  let _guard = init_log();
  // 测试代码执行
  run();
}

fn run() {
  // 这里使用log库的错误信息 tracing不会收集
  log::error!("无视log错误");
  // tracing相关标准信息输出
  tracing::warn!(
    "Tracing 警告信息 {} {}",
    "蓝色文本".blue(),
    "红色文本".red()
  );
  tracing::error!("Tracing 错误信息!!");
}

///
/// 日志配置初始化
///
fn init_log() -> tracing_appender::non_blocking::WorkerGuard {
  // 消费log门面日志 转为 tracing Event日志
  LogTracer::builder()
    // .with_max_level(log::LevelFilter::Error)
    .init()
    .expect("[PEAR] LogTracer 初始化失败");

  // 标准控制台输出layer
  let fmt_layer = fmt::layer()
    .with_level(true)
    // 指定标准控制台输出
    .with_writer(std::io::stdout)
    // 日志等级过滤
    .with_filter(LevelFilter::INFO);

  // 文件 appender 指定日志文件输出目录和文件名前缀
  // daily 指定生成文件名日期到年月日
  // 如： test-log.2023-08-30
  let file_appender = tracing_appender::rolling::daily("target/", "test-log");
  // 生成非阻塞写入器
  let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
  // 文件输出层
  let file_layer = fmt::layer()
    // 移除输出内容中的 颜色或其它格式相关转义字符
    .with_ansi(false)
    .with_writer(non_blocking)
    // 日志等级过滤
    .with_filter(LevelFilter::INFO);

  // 生成注册中心 Registry 绑定多个输出层
  let collector = tracing_subscriber::registry()
    .with(file_layer)
    .with(fmt_layer);

  // 订阅者全局注册
  tracing::subscriber::set_global_default(collector).expect("Tracing collect error");

  guard
}

```

## 输出结果

文件和控制台输出类似：

![tracing-log-content](https://cdn.oicnp.com/images/2023/tracing-log.png)

## 2023 年 12 月 5 日内容修正

- 增加 LogTracer 消费 log 日志
- 增加 with_filter 按日志等级过滤

## 日志监控相关内容链接

- [Tokio - Getting started with Tracing](https://tokio.rs/tokio/topics/tracing)
- [Tokio - Next steps with Tracing](https://tokio.rs/tokio/topics/tracing-next-steps)
- [Rust 语言圣经 75 - [日志与监控] 日志详解](https://zhuanlan.zhihu.com/p/496025829)
- [Rust 语言圣经 76 - [日志与监控] 门面即排场的 log](https://zhuanlan.zhihu.com/p/496027284)
- [Rust 语言圣经 77 - [日志与监控] 可咸可甜的 tracing](https://zhuanlan.zhihu.com/p/496028010)
- [Rust Lang Nursery - Config log](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html)
- [Command Line Applications in Rust - Output for humans and machines](https://rust-cli.github.io/book/tutorial/output.html)
