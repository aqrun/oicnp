---
title: 'Dioxus 踩坑之cli和wasm-bindgen版本冲突问题'
description: 'Dioxus-cli 依赖的 wasm-bindgen 版本锁定，和项目打包版本冲突'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'dioxus', 'wasm-bindgen']
---

## 问题

使用官网示例执行打包时报错，提示 wasm-bindgen 版本冲突：

```fish
dx build --features web
```

输出：

```
[INFO] 🚅 Running build command...
/ ⚙️ Compiling syn 2.0.38 (registry+https://github.com/rust-lang/crates.io-index)                                                                                                           [INFO] 👑 Build done.
[WARN] failed to parse `name` custom section Invalid name type (at offset 41143549)
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:

it looks like the Rust project used to create this wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust wasm file schema version: 0.2.88
     this binary schema version: 0.2.87

Currently the bindgen format is unstable enough that these two schema versions
must exactly match. You can accomplish this by either updating this binary or
the wasm-bindgen dependency in the Rust project.

You should be able to update the wasm-bindgen dependency with:

    cargo update -p wasm-bindgen --precise 0.2.87

don't forget to recompile your wasm file! Alternatively, you can update the
binary with:

    cargo install -f wasm-bindgen-cli --version 0.2.88

if this warning fails to go away though and you're not sure what to do feel free
to open an issue at https://github.com/rustwasm/wasm-bindgen/issues!
', /Users/aqrun/workspace/apps/rust/cargo/registry/src/mirrors.ustc.edu.cn-12df342d903acd47/dioxus-cli-0.4.1/src/builder.rs:129:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: 🚫 Building project failed: Build Failed: Bindgen build failed!
This is probably due to the Bindgen version, dioxus-cli using `0.2.81` Bindgen crate.
```

## 方案

解决方法也很简单，不使用 `--locked` 参数重新安装 cli 就可以

如果已经安装过一次要加 `--force` 强制更新

```fish
cargo install dioxus-cli --force
```

## 参考

- [Rust project used to create this wasm file was linked against version of wasm-bindgen that uses a different bindgen format #1601](https://github.com/DioxusLabs/dioxus/issues/1601)
- [Dioxus CLI wasm-bindgen version mismatch #1101](https://github.com/DioxusLabs/dioxus/issues/1101)
- [remove --locked from dioxus-cli installation instructions #153](https://github.com/DioxusLabs/docsite/pull/153)
