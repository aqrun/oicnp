---
title: '如何使用 Rust 从头构建一个区块链'
description: '2021年对加密货币、NFT和去中心化应用程序(DAPPs)来说是重要的一年，2022年将更重要。区块链是所有这些技术背后的底层技术。'

taxonomies:
  categories: ['rust', 'article']
  tags: ['rust', 'blockchain', 'crpto']
---

> [英文原文链接 https://coinsbench.com/how-to-build-a-blockchain-from-scratch-in-rust-9cedb59f8897](https://coinsbench.com/how-to-build-a-blockchain-from-scratch-in-rust-9cedb59f8897)

2021 年对加密货币、NFT 和去中心化应用程序(DAPPs)来说是重要的一年，2022 年将更重要。区块链是所有这些技术背后的底层技术。

区块链技术有潜力改变我们生活的几乎每一个方面，包括金融行业、旅游和出行、基础设施、医疗保健、公共部门、零售、农业和采矿、教育、通信、娱乐等。

> 世界上每个我崇拜的聪明人都有一个原因。他们明白这是第四次工业革命的驱动力: 蒸汽机、电力，然后是微芯片——第四次是区块链和加密货币。 —— 布洛克 皮尔斯（Brock Pierce）

## 什么是区块链？

区块链是跨点对点网络的去中心化交易账本，您也可以将区块链看作是不可变的去中心化数据库。一个区块链可以从根本上分解为几个组件，如节点、交易、区块、链和共识协议(工作证明、权益证明、历史证明)。

如果你像我一样，喜欢通过实战来学习。那么本文通过使用 Rust 构建一个区块链，会让你对区块链如何工作的有个基本概念。

听起来还不错是吧？那让我们开始吧。

## 现在开始

让我们从创建一个新的 Rust 项目开始：

```bash
cargo +nightly new blockchain
```

然后切换到你刚创建的目录中：

```bash
cd blockchain
```

然后为构建区块链添加必要的依赖包：

```toml
[dependencies]
chrono = "0.4"
serde = { version = "1.0.106", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.10.0"
```

下一步，创建 models 目录来保存你的区块链的大部分逻辑。目录中添加两个文件 `blockchain.rs` 和 `block.rs`。

两个文件中引入下面的依赖包并保存他们：

```rust
// Blockchain.rs
use chrono::prelude::*;
// internal module
use super::block::Block;
```

```rust
// Block.rs
use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
```

可能你已经注意到在 `blockchain.rs` 文件中引入了 `use super::block::Block;`， 这里我们只是引入了 `block.rs` 文件中的结构体，不用担心后面我会解释。

在引入必要的依赖包之后，让我们在 `blockchain.rs` 文件中定义一个 `Blocks` 类型:

```rust
type Blocks = Vec<Block>;
```

下一步，在 `blockchain.rs` 中创建 `Blockchain` 类型，并添加一个空的实现：

```rust
// Blockchain 代表区块链的结构体
#[derive(Debug)]
pub struct Blockchain {
    // 添加到链中的第一个区块
    pub genesis_block: Block,
    // 存储区块
    pub chain: Blocks,
    // 验证一个区块需要的最小工作量
    pub difficulty: usize,
}

impl Blockchain {}
```

下一步， 在 `block.rs` 文件中定义 `Block` 类型，并添加空的实现：

```rust
// Block 代表区块链中的区块结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // 保存的当前区块的索引
    pub index: u64,
    // 当前区块的创建时间
    pub timeStamp: u64,
    // 区块的工作证明
    pub proof_of_work: u64,
    // 前一个区块哈希
    pub previous_hash: String,
    // 当前区块哈希
    pub hash: String
}

impl Block {}
```

## 创建创世区块

创世区块是区块链中创建的第一个区块。接下来创建一个函数可以为我们的区块链生成一个创世区块，并返回一个新的 `Blockchain` 类型。

在 `blockchain.rs` 中添加我们的 `Blockchain` 实现，代码如下：

```rust
imple Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // 链中的第一个块
        let mut genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default()
        };
        // 从创世链开始创建链
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        // 创建一个区块链实例
        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty
        };
        blockchain
    }
}
```

上面的代码主要实现了这些功能：

- 创建我们的 `genesis_block` 实例
- 将我们创建的 `genesis_block` 添加到 `Blockchain` 类型的链中
- 返回一个 `Blockchain` 类型的实例

在我们创建的 `genesis_block` 实例中，注意到我们设置 previous_hash 的值为空字符串（String::default()），这是因为创世区块是区块链中的第一个区块所以不存在上一个区块。

还要注意，我们将 `genesis_block` 的哈希值设为一个空字符串("")，这是因为我们还没有为我们的创世区块计算哈希值。

## 为区块生成哈希值

哈希是根据当前区块的信息加密生成的。

在 `block.rs` 文件中为我们的区块实现添加 `calculate_hash()` 函数：

```rust
// 计算区块哈希
pub fn calculate_hash(&self) -> String {
    let mut block_data = self.clone();
    block_data.hash = String::default();
    let serialized_block_data = serde_json::to_string(&block_data).unwrap();

    // 计算生成 SHA-256 哈希值
    let mut hasher = Sha256::new();
    hasher.update(serialized_block_data);
    let result = hasher.finalize();
    format!("{:x}", result)
}
```

上面的代码中，实现了如下功能：

- 区块数据转为 JSON 格式
- 使用 SHA256 加密算法计算区块数据的哈希
- 返回 16 进制格式的哈希结果

## 创建一个新的区块

很棒！我们已经实现了创建创世区块和计算区块哈希的功能。

现在让我们在 blockchain.rs 文件中给 `Blockchain` 类型的实现添加创建新区块的功能：

```rust
pub fn add_block(&mut self, nonce: String) {
    let new_block = Block::new(
        self.chain.len() as u64,
        nonce,
        self.chain[&self.chain.len() - 1].previous_hash.clone()
    );
    new_block.mine(self.clone());
    self.chain.push(new_block.clone());
    println!("New block added to chain -> {:?}", new_block);
}
```

这里我们做了如下操作：

- 添加了 `add_block` 函数，形参是 &mut self(`Blockchain` 类型的实例)
- 创建 `Block` 类型的实例
- 使用 `Block` 类型的挖矿函数挖崛出一个区块哈希
- 将新区块添加到区块链中

接下来在 `block.rs` 文件中给 `Block` 类型实现添加如下代码：

```rust
// 创建一个新区块，会自动计算并设置哈希
pub fn new(
    index: u64,
    previous_hash: String,
) -> Self {
    // 当前要创建的区块
    let mut block = Block {
        index: 0,
        timestamp: Utc::now().timestamp_millis() as u64,
        proof_of_work: u64::default(),
        previous_hash: String::default(),
        hash: String::default(),
    };

    block
}
```

这里做了如下操作：

- 添加 `new()` 函数接收两个参数 index 和 previous_hash
- 创建 `Block` 类型的实例
- 为我们的区块生成区块哈希
- 最后返回 `Block` 类型的实例

## 挖崛新区块

我们已经成功实现了创建一个新区块。

让我们继续添加功能来挖崛新区块。挖掘新区块的过程包括生成一个以指定个数的 0 开头的 SHA256 哈希，这将是挖掘新区块时矿工必须解决的挖掘难度。

接下来在 `Block` 类型的实现中添加一个新函数

```rust
// 挖崛区块哈希
pub fn mine(&mut self, blockchain: Blockchain) {
    loop {
        if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
            self.proof_of_work += 1;
            self.hash = self.generate_block_hash();
        } else {
            break;
        }
    }
}
```

干得漂亮，目前为止我们已经实现了我们的区块链，现在让我们测试它。

让我们在 models 目录中新增 `mod.rs` 文件，并写入如下代码：

```rust
pub mod block;
pub mod blockchain;
```

这里主要是将我们早些时候添加的 `blockchain.rs` 和 `block.rs` 文件在 main.rs 文件中可以公开访问。

现在给 main.rs 文件添加如下代码：

```rust
mod models;

fn main() {
    let difficulty = 1;
    let mut blockchain = models::blockchain::Blockchain::new(difficulty);
    models::blockchain::Blockchain::add_block(&mut blockchain);
}
```

## 小结一下

通过本文你已经了解了如何使用 rust 从头创建一个简单的区块链。

我希望您喜欢阅读本文，您可以在[这里](https://github.com/ECJ222/Rust-blockchain)获得这个 Rust 区块链的完整源代码。

任何问题欢迎留言评论。
