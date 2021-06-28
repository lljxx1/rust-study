# Rust-Study
Rust学习路径规划和相关学习资料、开源项目汇总

## 基础
- [Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/title-page.html) - 语言基础学习
- [Rust By Example](https://rustwiki.org/zh-CN/rust-by-example)

## 教程
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [embedded operating systems](https://github.com/dddrrreee/cs140e-20win/)
- [Tutorial for rCore OS](https://github.com/rcore-os/rCore-Tutorial)
- [Roguelike Tutorial](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html) - 游戏开发教程


### 练习题
- [mingrep](https://kaisery.github.io/trpl-zh-cn/ch12-00-an-io-project.html)
- [webserver](https://kaisery.github.io/trpl-zh-cn/ch20-00-final-project-a-web-server.html)

## 开源学习

- [sonic](https://github.com/valeriansaliou/sonic) - 快速、轻量级搜索引擎
- [arrow-rs](https://github.com/apache/arrow-rs) - Apache Arrow的Rust实现
- [indradb](https://github.com/indradb/indradb) - 图数据库
- [tokio](https://github.com/tokio-rs/tokio) - 异步IO、网络、定时器基础库
- [rucene](https://github.com/zhihu/rucene) - Rust port of Lucene
- [terarkdb](https://github.com/bytedance/terarkdb) - 兼容RocksDB协议且性能更好的KV存储引擎
- [tikv](https://github.com/tikv/tikv) - 分布式KV数据库

## Roadmap
- 基于Rust的EVM，尝试兼容到基于rust写的公链
- 兼容Redis协议的单机key-value store
- 基于rucene实现一个类elasticsearch的全文检索
- 根据基于faiss-rs给搜索引擎扩充向量检索支持
- 玩具级的olap数据库

## KV数据
参考mini-redis实现一个kv数据库
- [mini-redis](https://github.com/tokio-rs/mini-redis)

## 搜索引擎
参考知乎的rucene或sonic
