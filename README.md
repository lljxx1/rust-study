# Rust-Study
rust学习

## 基础

### Rust 程序设计语言
[在线中文](https://kaisery.github.io/trpl-zh-cn/title-page.html)

### 练习
- [mingrep](https://kaisery.github.io/trpl-zh-cn/ch12-00-an-io-project.html)
- [webserver](https://kaisery.github.io/trpl-zh-cn/ch20-00-final-project-a-web-server.html)

## 开源学习
- [sonic](https://github.com/valeriansaliou/sonic) - 快速、轻量级搜索引擎
- [arrow-rs](https://github.com/apache/arrow-rs) - Apache Arrow的Rust实现
- [indradb](https://github.com/indradb/indradb) - 图数据库
- [tokio](https://github.com/tokio-rs/tokio) - 异步IO、网络、定时器基础库


## 长期Roadmap
- 兼容redis协议的key-value store，支持多节点路由分片等
- 基于rucene实现一个类elasticsearch的全文检索
- 根据基于faiss-rs给搜索引擎扩充向量检索支持
- 玩具级的olap数据库



### Key-Value
可以参考多个项目
- [tokio](https://github.com/tokio-rs/mini-redis)

### 搜索引擎
知乎的rucene或sonic