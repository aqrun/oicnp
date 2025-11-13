---
title: 'Rust 语言打造带P2P网络的区块链'
description: '2021年对加密货币、NFT和去中心化应用程序(DAPPs)来说是重要的一年，2022年将更重要。区块链是所有这些技术背后的底层技术。'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', 'blockchain', 'crypto', 'p2p']
---

> [英文原文链接 https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/](https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/)

当我们想到 P2P 技术和它现在的用途时，不可能不想到区块链技术。在过去十年中，IT 领域很少有话题像区块链技术和加密货币那样被大肆炒作或引发争议。

虽然人们对区块链技术的广泛兴趣变化很大——这自然是由于一些更广为人知和使用的加密货币背后的货币潜力——但有一件事是明确的:它仍然具有相关性，似乎不会有什么变化。

在前一篇文章中，我们讨论了如何在 Rust 中构建一个非常基本的、可以工作的(尽管效率很低)点对点应用程序。在本教程中，我们将演示如何使用 500 行 Rust 代码构建一个区块链应用程序，其中包含基本的挖掘方案、共识和点对点网络。

我们将详细介绍以下内容：

- 为什么区块链令人兴奋
- 使用 Rust 编写一个区块链应用
- 设置我们的 Rust 应用
- 区块链基础
- 区块，区块，区块
- 使用哪一个链？
- 挖矿
- 点对点基础
- 处理传入消息
- 将它整合到一起
- 测评我们的 Rust 区块链

## 为什么区块链令人兴奋

虽然我个人对加密货币或金融赌博一般不是特别感兴趣，但我发现对我们现有基础设施的部分去中心化的想法非常有吸引力。目前有许多基于区块链的伟大项目，旨在解决如气候变化、社会不平等、隐私和政务公开等社会问题。

潜在背后的技术是建立在安全、完全透明、去中心化账本理念的基础上，使参与者无需首先建立信任就可以进行互动，这似乎是一种改变游戏规则的技术。前面提到的那些雄心勃勃的想法中，有任何一个能够实现，获得支持，并取得成功，都将是令人兴奋的。

简而言之，区块链技术令人兴奋，不仅因为其改变世界的潜力，而且从技术角度来看也是如此。从点对点网络上的密码学到花哨的共识算法，这个领域有相当多迷人的主题可以深入研究。

## 使用 Rust 编写一个区块链应用

本文我们会使用 Rust 从头开始构建一个简单的区块链应用。我们的应用将会是非常的不高效、不安全也不健壮，但它可以帮助你理解如何以一种简单的方式实现众所周知的区块链系统背后的一些基本概念，并解释它们背后的一些思想。

我们不会深入每一个概念的每一个细节，本方案实现中也会有一些严重的缺陷。不是希望你将此项目用于任何生产环境中，而主要目的是构建一些你可以使用的东西，加入你自己的想法，并测试，以便从总体上更熟悉 Rust 和区块链技术。

重点将放在技术部分——即如何实现一些概念，以及它们如何一起发挥作用。我们不会解释区块链是什么，也不会涉及挖矿、共识等本教程所必需的内容。我们主要关心的是如何在 Rust 中使用最简化的方式实现这些想法。

此外，我们不会构建一个加密货币或类似的系统。我们的设计要简单的多：网络中的每个节点都可以通过本地挖掘一个有效的区块，然后广播该区块，向去中心化分类账本（区块链）添加数据（字符串）。

只要它是一个有效的区块(稍后我们将看到这意味着什么)，每个节点都会将该区块添加到其链中，我们的数据块就会成为分散的、防篡改的、不可破坏的(除了在我们的示例中所有提示都关闭)网络的一部分!

很明显，这是一个非常简化的设计，并且在一定程度上是人为设计的，在扩展时将很快遇到效率和健壮性问题。但既然我们做这个练习是为了学习，那完全没问题。如果你坚持到最后，并且有了一些动力，你可以把它扩展到任何你想要的方向，也许从我们微不足道的开始建立起下一个伟大的东西——你永远不知道!

## 设置我们的 Rust 应用

要进行下面的操作，你需要安装[最新的 Rust](https://www.rust-lang.org/tools/install)

首先创建一个新的 Rust 项目：

```bash
cargo new rust-blockchain-example
cd rust-blockchain-example
```

下一步，编辑 `cargo.toml` 文件并添加你需要的依赖包：

```rust
chrono = "0.4"
sha2 = "0.9.8"
serde = {version = "1.0", features = ["derive"]}
serde_json = "1.0"
libp2p = {version = "0.39", features = ["tcp-tokio", "mdns"]}
tokio = {version = "1.0", features = [
    "io-util", "io-std", "macros", "rt", "rt-multi-thread",
    "sync", "time"
    ]}
hex = "0.4"
once_cell = "1.5"
log = "0.4"
pretty_env_logger = "0.4"
```

我们使用 [libp2p](https://github.com/libp2p/rust-libp2p) 作为点对点网络层以及 [Tokio](https://github.com/tokio-rs/tokio) 作为底层的运行时。

`sha2` 库用于 sh256 哈希计算，`hex` 库用于将二进制的哈希转为可读以及便于传输的 16 进制格式。

除此之外的都是一些工具方法，如 `serde` 处理 JSON，`log` 和 `pretty_env_logger` 处理日志，`one_cell` 处理静态初始化，`chrono` 处理时间。

设置完成后，让我们首先实现区块链基础，然后，再将所有内容放到 p2p 联网的上下文中。

## 区块链基础

首先为我们的区块链定义数据结构：

```rust
pub struct App {
    pub blocks: Vec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}
```

就这些，并没有太复杂的东西。`App` 结构体保存我们的应用状态。本例中我们不会持久化存储区块链，因此一旦停止应用数据就会丢失。

这里的状态只是一个 `Blocks` 列表。我们会在列表结尾添加新的区块，这实际上就是我们的区块结构。

实际的逻辑将使这个区块列表成为一个区块链，应用中会实现其中每个区块引用前一个区块的哈希。尽可能构建一个支持我位需要的开箱即用验证的数据结构，这种方法似乎更简单，我们的目标就是简单。

在我们的示例中 `Block` 包含一个 `id` 索引从 0 开始垒加。然后是一个 sha256 加密哈希（随后我们会深入这个算法），是前一个区块的哈希，一个时间戳，以及保存在区块中的数据和一个随机数，在我们谈论挖崛（mining）区块时还会再涉及。

开始挖崛之前，让我们首先实现一些需要的验证函数，以保持数据状态的一致性和一些基本的共识，以便每个客户端都知道哪个区块链是正确的，以防有多个冲突的区块链。

从实现我们的 `App` 结构体开始：

```rust
impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::new().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43"
        };
        self.blocks.push(genesis_block);
    }
}
```

我们用一个空的链初始化我们的应用。随后我们会实现一些逻辑。在启动时我们会询问其它链如果他们的更长，就使用他们的。这是我们最简单的共识场景。

`genesis` 方法在我们的区块链中创建第一个数据写死的区块。这是一个特殊的区块他没有遵循和其它区块一样的规则。例如由于在他之前没有区块了，他没有一个有效的 `prevous_hash`。

我们需要他来“引导”我们的节点——或者，在第一个节点启动时引导整个网络。这个链就是从这里开始的。

## 区块，区块，区块

接下来，让我们实现能给链添加新区块的功能。

```rust
impl App {
    // ...

    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is at least on block");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("could not add block - invalid");
        }
    }
}
```

这里我们获取链的最后一个区块——我们的 `上一区块` ——然后验证我们要添加的区块是否合规。如果验证不通过我们只是输出错误日志。

在我们的简易应用中，我们不会实现任何真实的错误处理。正如你稍后将看到的，如果我们在节点之间遇到竞争条件的问题，并且有一个无效的状态，那么我们的节点直接崩溃。

我会对这些问题提及一些可能的解决方案，但这里我们不会实现他们。我们有相当多的内容要讲，即使忽略这些恼人的现实问题。

接下来看一下 `is_block_valid`，我们逻辑中的核心片段。

```rust
const DIFFICULTY_PREFIX: &str = "00";

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&fromat!("{:b}", c));
    }
    res
}

impl App {
    // ...

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"),
        ).start_with(DIFFICULTY_PREFIX) {
            warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash {
            warn!("block with id: {} has invalid hash", block.id);'
            return false;
        }
        true
    }
}
```

我们首先定义了常量 `DIFFICULTY_PREFIX`。这是我们简易版挖矿方案的基础。本质上，当挖掘一个区块时，挖掘人员必须对该块的数据进行哈希（在我们的例子中使用 SHA256），并找到一个以 00（两个 0）开头的哈希值。这也表示我们在网络上的“难度”。

可以想象，如果我们需要 3 个、4 个、5 个甚至 20 个前导零，那么找到合适哈希值的时间会增加很多。在一个真正的区块链系统中，这个困难将是一个网络属性，它是节点之间基于共识算法和网络的哈希能力达成一致的，因此网络可以保证在一定时间内产生一个新的区块。

本例我们不会处理这些，为了简单起见，我们将其硬编码为两个前导零。这样在普通硬件上也不会花太多的计算时间，因此测试时也不用担心需要等很久。

接下来是一个帮助函数，它只是以字符串的形式对给定的数组进行二进制表示
