---
title: 'Emacs 编辑器配置 Rust 开发环境'
description: 'Emacs 编辑器 Rust 语言开发环境配置， 过去的两年时间 Emacs 对 Rust 支持有了很大的提升。本文主要配置 Emacs 开发环境，功能如下：源代码导航（跳转到实'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', 'emacs']
---

> 原文链接：[https://robert.kra.hn/posts/2021-02-07_rust-with-emacs/](https://robert.kra.hn/posts/2021-02-07_rust-with-emacs/)。翻译有错漏欢迎评论区指正吐槽 😂。

![demo](https://cdn.oicnp.com/images/2023/demo.png)

过去的两年时间 Emacs 对 Rust 支持有了很大的提升。本文主要配置 Emacs 开发环境，功能如下：

- 源代码导航（跳转到实现、引用列表、模块大纲）
- 代码补全
- 代码片段
- 错误和警告行内高亮
- 代码修复和重构
- 自动导入定义（如特性）
- rustfmt 代码格式化
- 构建和运行其它 cargo 命令

本配置基于 [rust-analyzer](https://rust-analyzer.github.io/)，这是一个处于活跃开发状态并使 VS Code 支持 Rust 的 LSP 服务。

本文可以做为参考或直接去 [Github 仓库](https://github.com/rksm/emacs-rust-config) 获取源码直接运行（如下）。已测试可行的环境：Emacs 27.1、rust stable 1.49.0、macOS 11.1、Ubuntu 18.4、Win10。

对于想了解 Emacs-racer 的相关配置可以查看 [David Crook 的指南](https://github.crookster.org/my-emacs-rust-language-config/)。

内容目录：

- 快速开始
- 前置需求
  - Rust
  - rust-analyzer
  - Emacs
- Rust Eamcs 详细配置
  - rustic
  - lsp-mode 和 lsp-ui-mode
  - 代码导航跳转
  - 代码操作
  - 代码补全和片段
  - 行内错误
  - 行内类型提示
  - 附加包
- Debug 调试
- 感谢

## 快速开始

如果你已经安装了 Rust 和 Emacs 那可以直接快速开始而不用对现有配置做任何修改。可以使用如下命令在启动 Emacs 时加载[rksm/emacs-rust-config github 仓库](https://github.com/rksm/emacs-rust-config) 的 `standalone.el` 配置文件：

```bash
git clone https://github.com/rksm/emacs-rust-config
emacs -q --load ./emacs-rust-config/standalone.el
```

此命令会在启动 Emacs 时使用检出仓库的目录的 `.emacs.d` 路径（以及不同的 elpa 文件夹）。意味着不会使用和修改你原有的 `$HOME/.emacs.d`。如果你不确定或是很清楚这里描述的内容，这种方式都是最简单的配置。

所有的依赖都会在第一次启动时被安装，也就是第一次启动会多花些时间。

Windows 系统可以在快捷方式中添加这些参数启动 Emacs。如果是 macOS 并且安装的是 Emacs.app 则需要使用如下命令行：

```bash
/Applications//Emacs.app/Contents/MacOS/Emacs -q --load ./emacs-rust-config/standalone.el
```

## 先决条件

开始配置 Emacs 前，请确保你的系统已经安装了下面这些软件：

### Rust

安装 Rust 工具链及 cargo，这些使用 [rustup](https://rustup.rs/) 很容易安装。安装稳定版的 rust 并确保 `.cargo/bin` 已经添加到环境变量，rustup 可以默认完成这些操作。rust-analyzer 依赖 Rust 源码，可以运行命令 `rustup component add rust-src` 进行安装。

### rust-analyzer

需要 rust-analyzer 服务的二进制包。可以参考 [rust-analyzer 手册](https://rust-analyzer.github.io/manual.html#rust-analyzer-language-server-binary)进行安装，有预编译好的二进制包。然而，由于 rust-analyzer 开发非常活跃，我通常是下载 github 仓库源码再自行编译。这种方式更便于升级版本（可能也需要降级）。

```bash
$ git clone https://github.com/rust-analyzer/rust-analyzer.git
$ cd rust-analyzer
$ cargo xtask install --server # 会安装 rust-analyzer 到 $HOME/.cargo/bin 目录
```

经常会发生新版不能正常运行的问题。这种情况我建议查看 [rust-analyzer 改动日志](https://rust-analyzer.github.io/thisweek)，日志包含链接到每周更新的 git 提交。如果不能正常运行，可以试着构建早一些的版本，或许可以成功。写本文时（2021.11.15）我用的是[7366833](https://github.com/rust-analyzer/rust-analyzer/commit/73668334f05c3446b04116ccc3156240d2d8ab19)，这个版本在 稳定版 Rust 1.56.1 以及 Ubuntu、MacOS 和 Windows 系统都工作正常。

### Emacs

我测试过可以配置的版本是 Emacs 27.1。Mac 上我通常使用 [emacsformacosx](https://emacsformacosx.com/)。Windows 上我使用 “附近的 GNU 镜像”链接为 [gnu.org/software/emacs](https://www.gnu.org/software/emacs/download.html)。在 Ubuntu 需要[添加第三方 apt 仓库](https://ubuntuhandbook.org/index.php/2020/09/install-emacs-27-1-ppa-ubuntu-20-04/)。注意此配置在较老的 emacs 版本也可以工作，但 Emacs 27 在 JSON 解析方面有实质性的改进大大提高了 LSP 客户端的速度。

注意，我使用 [use-package](https://github.com/jwiegley/use-package) 作为 Emacs 的包管理器。它将自动安装这个配置的独立版本。否则可以在你的 `init.el` 添加如下片段：

```lisp
(unless (package-installed-p 'use-package)
	(package-refresh-contents)
	(package-install 'use-package))
```

## Rust Emacs 详细配置

用到的模式有：

- rustic
- lsp-mode
- company
- yasnippet
- flycheck

### Rustic

[rustic](https://github.com/brotzeit/rustic) 是 `rust-mode` 的一个分支并扩展了很多有用的功能（可以查看它的 github readme）。它是配置的核心，如果你只需要代码高亮和 emacs 绑定的 cargo 快捷键，那就这一个就够了不需要其它任何 Emacs 扩展包。

```lisp
(use-package rustic
  :ensure
  :bind (:map rustic-mod-map
      ("M-j" . lsp-ui-imenu)
      ("M-?" . lsp-find-references)
      ("C-c C-c l" . flycheck-list-errors)
      ("C-c C-c a" . lsp-execute-code-action)
      ("C-c C-c r" . lsp-rename)
      ("C-c C-c q" . lsp-wordspace-restart)
      ("C-c C-c Q" . lsp-workspace-shutdown)
      ("C-c C-c s" . lsp-rust-analyzer-status))
  :confi
  ;; 减少闪动可以取消这里的注释
  ;; (setq lsp-eldoc-hook nil)
  ;; (setq lsp-enable-symbol-highlighting nil)
  ;; (setq lsp-signature-auto-activate nil)

  ;; 注释下面这行可以禁用保存时 rustfmt 格式化
  (setq rustic-format-on-save t)
  (add-hook 'rustic-mode-hook 'rk/rustic-mode-hook))

(defun rk/rustic-mode-hook ()
  ;; 所以运行 C-c C-c C-r 无需确认就可以工作，但不要尝试保存不是文件访问的 rust 缓存。
  ;; 一旦 https://github.com/brotzeit/rustic/issues/253 问题处理了
  ;; 就不需要这个配置了
  (when buffer-file-name
    (setq-local buffer-save-without-query t)))
```

rustic 的大部分功能都绑定到 `C-c C-c` 前缀（也就是按 Control-c 键两次再按其它键）：

![shortcut](https://cdn.oicnp.com/images/2023/rustic-shortcuts-1.png)

你可以使用 `C-c C-c C-r` 调用 `cargo run` 运行程序。有可能需要你指定一些参数例如使用发布模式运行可以指定 `--release` 或要运行名称为 "other-bin" 的目标程序使用参数 `--bin other-bin`（替换 mina.rs）。 要给可执行程序本身传递参数使用 `-- --arg1 --arg2`。

快捷键 `C-c C-c C-c` 会运行测试。非常方便执行内联测试而不用经常的来切回在终端和 Emacs 之间切换。

`C-c C-p` 命令会打开一个固定位置的弹出缓冲区显示上面的快捷命令。

Rustic 提供了一些和 cargo 很方便的集成，例如，`M-x rustic-cargo-add` 会允许你添加依赖到项目的 `Cargo.toml` （通过 [cargo-edit](https://crates.io/crates/cargo-edit) 这个需要提前安装好）。

如果你想分享代码片段，`M-x rstic-playpen` 命令会把你当前缓冲区在 [https://play.rust-lang.org](https://play.rust-lang.org) 打开，可以让你在线运行 Rust 代码并且有一个可以分享的链接。

默认启用了保存时使用 rustfmt 进行代码格式化。要禁用它可以设置 `(setq rustic-format-on-save nil)`。也可以在需要时使用 `C-c C-c C-o` 格式化缓冲区。

### lsp-mode and lsp-ui-mode

lsp-mode 提供了 [rust-analyzer](https://emacs-lsp.github.io/lsp-mode/page/lsp-rust/) 的集成。启用了一些 IDE 的功能如源代码导航、通过 flycheck （如下）语法检查错误高亮以及为 company 提供代码自动补全（如下）。

```lisp
(use-package lsp-mode
  :ensure
  :commands lsp
  :custom
  ;; 保存时使用什么进行检查，默认是 "check"，我更推荐 "clippy"
  (lsp-rust-analyzer-cargo-watch-command "clippy")
  (lsp-eldoc-render-all t)
  (lsp-idle-delay 0.6)
  (lsp-rust-analyzer-server-display-inlay-hints t)
  :config
  (add-hook 'lsp-mode-hook 'lsp-ui-mode))

(use-package lsp-ui
  :ensuer
  :commands lsp-ui-mode
  :custom
  (lsp-ui-peek-always-show t)
  (lsp-ui-sideline-show-hover t)
  (lsp-ui-doc-enable nil))
```

lsp-ui 是可选的，它提供在光标处标记并显示内联弹层以及光标处的代码修复。如果你发现它闪动不想开启这个功能，只需要移除 `:config (add-hook 'lsp-mode-hook 'lsp-ui-mode)`。

上面的配置也关闭了 lsp-ui 内联显示的文档功能。这个比较符合我的习惯，由于它经常遮住源代码。如果你也想关闭在 mini 缓冲区显示的文档可以添加 `(setq lsp-eldoc-hook nil)`。在光标移动时想操作的更少可以考虑 `(setq lsp-signature-auto-activate nil)` 和 `(setq lsp-enable-symbol-highlighting nil)`。

### Code Navigation

配置好 lsp-mode 当你的光标在一个标记上面时你就可以使用 `M-.` 来跳转到函数、结构体、包等的定义处。`M-,` 可以再跳回来。使用 `M-?` 你可以列出标记的所有引用。如下演示：

![rust-lsp-demo](https://cdn.oicnp.com/images/2023/rust-lsp-demo-1.gif)

使用 `M-j` 你可以打开允许你在函数和其它定义之间快速跳转的当前模块大纲。

![imenu](https://cdn.oicnp.com/images/2023/imenu.png)

### 代码操作（Code Actions）

可以使用 `M-x lsp-rename` 和 `lsp-execute-code-action` 进行重构。代码操作基本上就是代码转换和修复。例如代码检查可能会发现更优雅的代码表达方式：

![rust-lsp-demo-2](http://assets.oicnp.com/pic/rust-lsp-demo-2.gif)

可用的代码操作的数量还在持续增长。完整的列表可以查看 [rust-analyzer 文档](https://rust-analyzer.github.io/manual.html#assists-code-actions)。收藏的包括自动函数引入或完全的代码合格化，例如，一个模块还没有引入 HashMap，输入 `HashMap` 然后选择选项可以引入 `Import std::collections::HashMap`。其他代码操作允许你在匹配表达式中添加所有可能的分支，或者为定义实现转换 `#[derive(Trait)]` 为必要的的代码。还有很多很多。

如果你在开发宏，快速查看他们是如何扩展的将非常实用。使用 `M-x lsp-rust-analyzer-expand-macro` 或快捷键 `C-c C-c e` 来展开宏。

### 代码补全和片段（Code completion and snippets）

lsp-mode 直接和 Emacs 的补全框架 [company-mode](https://company-mode.github.io/) 集成。它会显示一个能被插入到光标处的可选符号列表。在使用不熟悉的库（或 std 库）时非常有用，不再需要经常查看文档。Rust 的类型系统被用作补全的来源，因此你可以插入有意义的内容。

默认代码补全弹框会在 `company-idle-delay` 设置的 0.5 秒后显示。你可以修改这个值或者设置 `company-begin-commands` 为 `nil` 来完全关闭弹层。

```lisp
(use-package company
  :ensure
  :custom
  (company-idle-delay 0.5) ;; 弹层延迟显示时长
  ;; (company-begin-commands nil) ;; 取消注释可以禁用弹层
  :bind
  (:map compnay-active-map
    ("C-n". company-select-next)
    ("C-p". company-select-previous)
    ("M-<". company-select-first)
    ("M->". company-select-last)))

(use-package yasnippet
  :ensure
  :config
  (yas-reload-all)
  (add-hook 'prog-mode-hook 'yas-minor-mode)
  (add-hook 'text-mode-hook 'yas-minor-mode)
)
```

这里也会通过 [yasnippet](https://joaotavora.github.io/yasnippet/) 启用代码片段。我有一个[常用片段 github 仓库](https://github.com/rksm/emacs-rust-config/tree/master/snippets/rustic-mode) 列表。可以随意拷贝并修改他们。他们的工作方式是通过输入固定的字符序列然后按 TAB 键。例如 `for<TAB>` 会展开为 for 循环。你可以自定义预填的内容和展开的停止数量甚至执行自定义的 elisp 代码。具体查看 yasnippet 文档。

要在点击 TAB 键时启用代码片段展开、代码补全和缩进，我们需要自定义在点击 TAB 时执行的命令：

```lisp
(use-package company
  ;; ... 接上面 ...
  (:map company-mod-map
    ("<tab>". tab-indent-or-complete)
    ("TAB". tab-indent-or-complete)
  )
)

(defun company-yasnippet-or-complete ()
  (interactive)
  (or (do-yas-expand)
    (company-complete-common))
)

(defun check-expansion ()
  (save-excursion
    (if (looking-at "\\_>") t
      (backward-char 1)
      (if (looking-at "\\.") t
        (backward-char 1)
        (if (looking-at "::") t nil)
      )
    )
  )
)

(defun do-yas-expand ()
  (let ((yas/fallback-behavior 'return-nil))
    (yas/expand)
  )
)

(defun tab-indent-or-complete ()
  (interactive)
  (if (minibufferp)
    (minibuffer-complete)
    (if (or (not yas/minor-mod)
          (null (do-yas-expand))
        )
        (if (check-expansion)
          (company-complete-common)
          (indent-for-tab-command)
        )
    )
  )
)
```

大部分常用片段是 `for`、`log`、`ifl`、`match` 和 `fn` 。

### 行内错误

这个很简单，rustic 做了很多繁重的任务。我位只需要确认代码检查已经加载：

```lisp
(use-package flycheck :ensure)
```

也可以执行 `M-x flycheck-list-errors` 或点击快捷键 `C-c C-c l` 来显示一个错误和警告的列表。

### 行内类型提示

Rust-analyzer 和 lsp-mode [可以显示行内类型注释](https://emacs-lsp.github.io/lsp-mode/page/lsp-rust/#inlay-hints)。通常当把光标放在定义的变量上时会通过 eldoc 进行显示，使用注释你可始终看到推断的类型。 使用 `(setq lsp-rust-analyzer-server-display-inlay-hints t)` 来启用它们。要真正的插入推断的类型到源代码，你可以移动光标到定义的变量并执行 `M-x lsp-execute-code-action` 或 `C-c C-c a`。

注意它们可能和 `lsp-ui-sideline-mode` 交互的不是很好。如果你只需要提示而想禁用边线模式（sideline mode），你可以给 `rustic-mode-hook` 添加 `(lsp-ui-sideline-enable nil)`。

## 代码调试

Emacs 通过 [dap-mode](https://emacs-lsp.github.io/dap-mode/) 集成了 gdb 和 lldb。为了设置支持 Rust 调试，你需要做一些额外的配置和构建步骤。特别是你需要有 `lldb-mi`(https://github.com/lldb-tools/lldb-mi)，它不包含在 Apple 通过 XCode 提供的官方 llvm 发行版里。

我只在 macOS 上测试编译了 `lldb-mi`。下面是我的操作步骤：

1. 通过 homebrew 安装 llvm 和 cmake
2. 检出 lldb-mi 代码库
3. 构建 lldb-mi 可执行文件
4. 将目录链接到我的 PATH

```bash
$ brew install cmake llvm
$ git clone https://github.com/lldb-tools/lldb-mi
$ mkdir -p lldb-mi/build
$ cd lldb-mi/build
$ cmake ..
$ cmake --build .
$ ln -s $PWD/src/lldb-mi /usr/local/bin/lldb-mi
```

为了让 Emacs 能找到可执行文件，你需要确保 `exec-path` 在启动时是正确配置的。完整的 dap-mode 配置如下：

```lisp
(use-package exec-path-from-shell
  :ensure
  : init (exec-path-from-shell-initialize)
)

(use-package dap-mode
  :ensure
  :config
  (dap-ui-mode)
  (dap-ui-controls-mode 1)

  (require 'dap-lldb)
  (require 'dap-gdb-lldb)
  ;; 安装 .extendsion/vscode
  (dap-gdb-lldb-setup)
  (dap-register-debug-template
    "Rust::LLDB Run Configuration"
    (list :type "lldb"
      :request "launch"
      :name "LLDB::Run"
      :gdbpath "rust-lldb"
      :target nil
      :cwd nil
    )
  )
)
```

`(dp-gdb-lldb-setup)` 会安装一个 VSCode 扩展到 `user-emacs-dir/.extension/vscode/webfreak.debug` 目录。我碰到有一个问题是这个安装不是经常会成功。如果最后你没有 "`webfreak.debug`" 目录你可能需要删除 `vscode/` 目录然后再执行 `(dap-gdb-lldb-setup)`。

我还需要执行一次 `sudo DevToolSecurity --enable` 来允许调试器访问进程。

另外还有一个问题是，当我启动调试目标时我会看到：

```
Could not start debugger process, does the program exist in filesystem?
Error: spawn lldb-mi ENOENT
```

即使 `lldb-mi` 在我的环境变量并且我可以在 Emacs 里面启动它。结果表明错误不是来自 `lldb-mi` 而是你启动目标的目录。当你使用 `M-x dap-debug` 或通过 `dap-hydra d d` 启动调试，然后选择 `Rust::LLDB Run Configuration` 时确保你想要调试的可执行目标的目录不是相对路径也不能包含 `~`。如果是绝对路径就应该可以工作。

如下可能会发生上面错误的失败（注意未展开的 `~/`）：

![dap-fail](https://cdn.oicnp.com/images/2023/dap-fail.png)

我需要指定完整的路径 `/Users/robert/projects/rust/emacs/test-project/target/debug/test-project`。

一旦成功执行看起来应该如下：

<video controls src="https://cdn.oicnp.com/images/2023/emacs-debugging-dap.mp4" style="max-height: 620px" />

上面示例我首先使用 `C-c C-c d` 激活 `dab-hydra`。然后使用 `d d` 选择 Rust 调试目标（提前使用 cargo 构建的）。在这之前还用 `d p` 设置了一个断点。然后我使用 `n` 和 `i` 在代码中步进。注意你也可以使用鼠标设置断点和步进。

配置调试并没有预期的顺畅，但一旦运行起来会非常有趣！

### Rust playground

你或许已经见识了在线的 Rust playgroud [https://play.rust-lang.org/](https://play.rust-lang.org/)，可以让快速运行和分享 Rust 代码片段。Emacs 有一个类似的允许你快速创建（或移除）Rust 草稿项目的项目是 `[grafov/rust-playgroud](https://github.com/grafov/rust-playground)`。默认 `rust-playgroud` 命令会在目录 `~/.emacs.d/rust-playgroud/` 创建 Rust 项目，并打开 `main.rs`，使用绑定的快捷键快速运行项目(`C-c C-c`)。这个非常便于你快速测试 Rust 代码片段或调试一个库。这一切都来自于你自己的编辑器！

### 附加包

这还有一些 emacs 包本文就不再细说了，会极大的提升使用 Emacs 进行 Rust 或其它语言开发的体验。如下：

- [projectile](https://github.com/bbatsov/projectile)：将项目的概念引入到 emacs 以及大量相关操作的命令。如在项目打开 shell、搜索项目代码等。
- [helm](https://emacs-helm.github.io/helm/)、[selctrum](https://github.com/raxod502/selectrum)、[ivy](https://github.com/abo-abo/swiper#ivy)：我们花了很多时间从列表中选择一个还是多个选项。让它可以打开文件、缓冲区间切换或执行命令（M-x）。所有这些包让在 emacs 中通过键盘输入来选择选项变得简单，并能够过滤大的列表。help 是我个人的日常驱动，但 selectrum 是一个更轻量的替代。它使用在相关的 github 项目的 standalone.el 版本中。
- [shackle](https://depp.brause.cc/shackle/)：Emacs 默认的窗口规则并不是最优的。Shakle 允许定义匹配缓冲区名称的规则。我默认的规则在[这个 gist](https://gist.github.com/rksm/8c07d9ccc9e15adf752d3dd73dd9a61e)。
- [dired](https://www.gnu.org/software/emacs/manual/html_node/emacs/Dired.html)：内置于 Emacs。你最后需要一个文件管理器。

## 感谢这些包的开发者们!

最后要说声谢谢！感谢所有本文中提到的开源软件的开发和维护者们。Rust-analyzer 项目是令人惊叹的，它极大的改善了 Rust Emacs 工具状态。当然也离不开非常有用的 lsp-mode 和 lsp-ui。rustic 简化了 rust-mode 模式相关的必要配置，并增加了非常有用的特性。在其它语言 company 和 flycheck 是我的默认配置。当然还要感谢所有 Emacs 的维护人员以及我记不太清的参与其中的所有人！

---

1. [Racer](https://github.com/racer-rust/emacs-racer) 曾经是配置 Emacs IDE 特性（代码导航等）的最佳选择。它是比 RLS 和 rust-analyzer 都快的非 LSP 解决方案。然而有很多有关代码补全的特性已经不如 rust-analyzer 了。
2. Emacs 也通过 [GUD](https://www.gnu.org/software/emacs/manual/html_node/emacs/GDB-Graphical-Interface.html) 内置了对 gdb 的支持， 但需要直接控制 gdb 进程。DAP 更类似于 LSP，因为它用于远程控制调试过程，使编辑器更容易集成它。
