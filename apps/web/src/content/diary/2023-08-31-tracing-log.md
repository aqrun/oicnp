---
title: 'Rust tracing-log 消费log信息'
description: 'Tracing 默认确实不会处理 log 库的日志信息。但查看文档是有说明如何进行信息转换,可以使用 tracing-log库提供的兼容层来消费Log记录'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'tracing', 'log', '日志']
---

昨天测试的 tracing 代码中有如下 log 使用方式：

```rust
// 这里使用log库的错误信息 tracing不会收集
log::error!("无视log错误");
```

Tracing 默认确实不会处理 log 库的日志信息。但查看文档是有说明如何进行信息转换，
[具体查看](https://docs.rs/tracing/latest/tracing/#consuming-log-records)

可以使用 [`tracing-log`](https://crates.io/crates/tracing-log) 库提供的兼容层，
来消费 Log 记录，会把 log 记录转为 [Tracing events](https://docs.rs/tracing/latest/tracing/event/struct.Event.html)

使用方式也很简单

在 init_log 中初始化 LogTracer 既可

```rust
use tracing_log::LogTracer;
use log;

fn ini_log() {
  LogTracer::init()?;
  // ... 其它代码
}

// 测试
// 正常在控制台也可以看到下面 log::info! 信息
// info!("这是log::info! 信息");
// tracing::info!("这是 tracing::info! 信息");
```
