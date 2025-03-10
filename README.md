# JL-transachain

**区块链项目**

## 项目介绍

JL-transachain 是一个基于 Rust
语言开发的区块链技术的交易系统，主要用于交易数据的存储和查询。系统采用区块链技末，将交易数据存储在区块链上，确保数据的不可篡改性。系统包含区块链、区块、交易等模块，提供了区块链数据的存储和验证功能。

## 项目结构

```
JL-transachain
├── README.md
└── src
    ├── blockchain.rs
    ├── block.rs
    ├── hashable.rs
    ├── lib.rs
    ├── main.rs
    ├── p2p
    │   ├── mod.rs
    │   └── p2p_node.rs
    ├── server
    │   ├── mod.rs
    │   └── server.rs
    └── transaction.rs
```

## 项目功能

- 区块生成：创建包含交易数据的合法区块
- 共识机制：实现简单 PoW 挖矿算法
- 网络接口交互

## 依赖

项目依赖以下库：

- `hex`：用于十六进制编码和解码
- `crypto-hash`：用于加密哈希计算
- `actix-web, actix-cors`：用于web端
- `serde, serde-json`：用于解析数据格式

## 安装与运行

1. 克隆项目到本地：

   ```sh
   git clone git@github.com:AlanKSorata/JL-transachain.git
   cd JL-transachain
   ```

2. 使用 Cargo 构建项目：

   ```sh
   cargo build
   ```

3. 运行项目：
   ```sh
   cargo run
   ```

## 代码说明

### `Block` 结构体

定义在 [`src/block.rs`](src/block.rs) 中，表示区块链中的一个区块。包含以下字段：

- `index`：区块索引
- `timestamp`：时间戳
- `hash`：区块哈希
- `prev_block_hash`：前一个区块的哈希
- `nonce`：随机数
- `transactions`：交易列表
- `difficulty`：挖矿难度

### `Blockchain` 结构体

定义在 [`src/blockchain.rs`](src/blockchain.rs) 中，表示区块链。包含以下字段：

- `blocks`：区块列表
- `unspent_outputs`：未花费输出集合

### `Transaction` 结构体

定义在 [`src/transaction.rs`](src/transaction.rs) 中，表示交易。包含以下字段：

- `inputs`：输入列表
- `outputs`：输出列表

### `Hashable` 特性

定义在 [`src/hashable.rs`](src/hashable.rs) 中，表示可哈希的对象。包含以下方法：

- `bytes`：返回对象的字节表示
- `hash`：返回对象的哈希值

### 工具函数

定义在 [`src/lib.rs`](src/lib.rs) 中，包括：

- `now`：获取当前时间戳
- `u32_bytes`：将 `u32` 转换为字节数组
- `u64_bytes`：将 `u64` 转换为字节数组
- `u128_bytes`：将 `u128` 转换为字节数组
- `difficulty_bytes_as_u128`：将字节数组转换为 `u128`

## 测试

使用 Cargo 运行测试：

```sh
cargo test
```

## web 接口调试

### 1. scan 接口

访问 web 提供的接口[`/scan`](http://localhost:8080/scan)

![data](images/1.png)

### 2. data 接口

访问 web 提供的接口[`/data`](http://localhost:8080/data)

![data](images/2.png)

### 3. mine 接口

访问 web 提供的接口[`/mine`](http://localhost:8080/mine)

![data](images/3.png)

