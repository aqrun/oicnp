---
title: 'Helix 编辑器插件系统 Scheme 方案锁定'
description: 'Rust 实现的TUI编辑器 - Helix。插件系统讨论贴#3806是22年9月12日开启的经过差不多一年收集了近320个回复和讨论。终于在23年8月10日初步锁定为Scheme实现方案。'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'helix', '编辑器', '插件']
---

![Helix Editor Screenshot](https://pic.imgdb.cn/item/65084935204c2e34d3a72b92.png)

Rust 实现的 TUI 编辑器 - [Helix](https://helix-editor.com/)。插件系统讨论贴[#3806](https://github.com/helix-editor/helix/discussions/3806)
是 22 年 9 月 12 日开启的经过差不多一年收集了近 320 个回复和讨论。终于在 23 年 8 月 10 日初步锁定实现方案。

<br/>

## 各种实现方案都有提出

如

- wasm
- scheme
- deno
- 纯 Rust 插件（云构建）
- 解释型 rust(如 iRust)
- lua
- lisp 家族
- rahi
- js
- python

等等，最终主开发者 [archseer](https://github.com/archseer) 冻结了此讨论贴，
基本上确认是采用了类 Scheme 的实现方案。
[原回复贴内容链接](https://github.com/helix-editor/helix/discussions/3806#discussioncomment-6686976)。

<br/>

## 具体理由大体如下：

<br/>

archseer 后面还会再出一个详细的文章，详述已经考虑过的替代方案(包括 WASM)和所选择的设计约束，
但目前倾向于类 scheme 的实现方式(并且 @mattwparas 在[#3806 回复](https://github.com/helix-editor/helix/discussions/3806#discussioncomment-6064568)
中已经实现了一个基本的原型，甚至涵盖了类似 vim-dirvish 的文件树，这是目前插件系统实现比较完善的项目)。

<br/>

### Scheme 的优势

- 从维护的角度来看，Scheme 是一种非常小的语言(您可以一次阅读完整的规范)，可以很方便把编译器和 VM 嵌入编辑器内核。
- 从长期维护 Helix 考虑，使用一个我们完全理解的实现会有非常多的好处(也不必担心语言会消亡)。
- Scheme 语法或许不是每个人都会喜欢，但她非常容易解析，没有歧义，可以很容易地从更高级别的语言编译目标。
- 宏系统非常灵活且富有表现力。Elisp 是 emacs 如此流行和可扩展的主要原因之一。

<br/>

Helix 是一个实用的编辑器: 她应该像你期望的那样开箱即用，不期望用户编写大量的 Scheme，
除非他们是插件作者。事实上，编辑器核心已经有了足够的可扩展性，还没有真正需要一个插件系统。
但仍然希望有一个足够灵活的插件系统，以扩展编辑器以适应不太常见的用例。

<br/>

### 为什么不使用 WASM

WebAssembly 很流行，但它并不是万能的解决方案。我们需要向该语言公开一个非常大的 ABI，
而且由于没有跨语言兼容的内存布局，我们最终还是会被锁定在运行在 WASM 之上的单一语言中
(或者承担支持其他语言的多个 sim 的维护负担)。我们只会得到 WASM 的 VM 的好处，
也不得不导入一个比编辑器本身还要大很多数量级的项目。暂时还不清楚这种权衡是否值得。

<br/>

即使我们能够通过 WASM 支持多种语言，对我来说，专注于一种语言似乎更好，这样生态系统就不会分裂成更小的部分，
如果每个插件都可能使用不同的语言，会使插件贡献变得更加困难。

<br/>

### 为什么不选择编译形式的插件

脚本语言也应该用于配置。虽然有软件可以做到这一点(例如 dwm)，
但它给用户设置了一个障碍: Helix 需要 Rust 知识，切换配置也可能会导致几分钟的编译时间。
