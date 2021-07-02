## Rust从入门到放弃
> 最近学了大概有10来天rust了，已经手抄了好几个小项目的代码。跃跃欲试给自己找了几道题目玩玩，还挺好玩的hh

## qa.rs
``` rust
#![deny(warnings)]

fn main() {
    let i = 0i64;
    change_value();
    assert_eq!(i, 1);
}

//
// Implement this function to run a successful `cargo run --release`.
//
// **NOTE**
// - do NOT change any existing codes except that `todo!()`
//
fn change_value() {
    todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let mut a = Vec::new();
        {
            // fix this line to make this test pass
            a[10000000] = 1;
        }
        assert_eq!(a[10000000], 1);
    }

    #[test]
    fn test2() {
        let a = async { "Hello World!" };
        let b;
        {
            // fix this line to make this test pass
            b = a();
        }
        assert_eq!(b, "Hello World!");
    }
}
```

这道题的话就是要保证`cargo run --release`正常执行

大概三个问题：
- main函数里的i变量需要在`change_value`修改为1，assert_eq!才能顺利执行
- a数组第10000000个值设为1
- 同步块里调用async函数a


### 第一题
``` rust
fn main() {
    let i = 0i64;
    change_value();
    assert_eq!(i, 1);
}

fn change_value() {
    todo!()
}
```

按照我浅薄的理解，`change_value`里面是无法访问`main`块里作用域的，如果不把`i`传参进去，
要想把`i`改为1是不太可能的。如果只能在`change_value`里加逻辑的话理论上是做不到覆盖`i`的值。如果可以修改外面的代码话`let mut = i; change_value(&mut i);`即可。

复习了很多作用域的定义，还google了很多可能可以的办法，结论应该是不行...  
但回到题目的要求：可以让这个程序正常运行，突然想到前几天写`minigrep`的时候用到的中断`process::exit`，既然`i`值理论上无法修改，但我可以在这个函数块里让进程停止运行，这样一来后面的`assert_eq`也就不会继续运行了hhh

### 第二题
``` rust
let mut a = Vec::new();
{
    // fix this line to make this test pass
    a[10000000] = 1;
}
```
报错`index out of bounds: the len is 0 but the index is 10000000`  
翻[Vec的文档](https://doc.rust-lang.org/std/vec/struct.Vec.html)可以看到有个resize的方法

``` rust
let mut a = Vec::new();
{
    // fix this line to make this test pass
    a.resize(10000001, 0);
    a[10000000] = 1;
}
```
在前面加个`resize`，确保长度大于下面要赋值长度的

### 第三题
``` rust
let a = async { "Hello World!" };
let b;
{
    // fix this line to make this test pass
    b = a();
}
assert_eq!(b, "Hello World!");
```
报错`error[E0618]: expected function, found impl Future`  
这是要在同步函数里调用异步函数hh，google了下找到篇文章用了`block_on`

``` rust
let a = async { "Hello World!" };
let b;
{
    // fix this line to make this test pass
    b = futures::executor::block_on(a);
}
```
这里引入了依赖库`futures`, 也试着找了下`block_on`最简单的实现看下能不能在不引入依赖的情况下自己实现`block_on`，但都看了下好复杂...


# FunDB
第二道大题是个结构化的项目了里面有很多实现被抹去了，需要自己完善细节。  
但实际的话这个项目相似度很高很多逻辑是重复相似的，仔细看很多实现都能在里面找到解，整体来讲都不难学习意义也挺大的。

题目：
- 链接: https://pan.baidu.com/s/1yHGHDk70oEhEWbunE46iqw 提取码: vynm 

1. implement all `todo!()` in the codebase(stay relaxed, they are all very simple)
    ```
    src/helper.rs:155:        todo!()
    src/helper.rs:164:        todo!()
    src/helper.rs:173:        todo!()
    src/helper.rs:182:        todo!()
    src/helper.rs:191:        todo!()
    src/helper.rs:200:        todo!()
    src/helper.rs:209:        todo!()
    src/helper.rs:218:        todo!()
    src/helper.rs:228:    todo!()
    src/helper.rs:233:    todo!()
    src/helper.rs:238:    todo!()
    src/mapx/backend.rs:125:        todo!()
    src/mapx/backend.rs:180:        todo!()
    src/mapx/backend.rs:190:        todo!()
    src/mapx/backend.rs:215:        todo!()
    src/mapx/mod.rs:165:        todo!()
    src/vecx/backend.rs:113:        todo!()
    src/vecx/mod.rs:134:        todo!()
    src/vecx/mod.rs:166:        todo!()
    src/vecx/mod.rs:185:        todo!()
    ```

## 分析
扩展了两种的支持持久化的数据结构，持久化是用rust写的`sled`嵌入式数据库做的
 - Mapx
 - Vecx

为了提高读取性能，会在内存中缓存一定量的数据，超出的部分需要从`sled`取  
整个代码主要是围绕实现这两个数据类型的，以及各种操作和运算+迭代器等相关操作的实现

## helper.rs

`Value`对象的相关trait实现

### Trait Deref Value
实现 Deref trait 允许我们重载 解引用运算符  
详细：Rust 程序设计语言 - [通过 Deref trait 将智能指针当作常规引用处理](https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html)

#### `src/helper.rs:155:        todo!()` return Value reference  
``` rust 
fn deref(&self) -> &Self::Target {
    &self.value
}
```

### Trait PartialEq + PartialOrd
等值比较和次序比较的运算符重载具体实现
#### `src/helper.rs:164:        todo!()` other type is Value
``` rust
fn eq(&self, other: &Value<'a, V>) -> bool {
    self.value == other.value
}
```
另一个比对对象也是Value类型

#### `src/helper.rs:173:        todo!()` other is reference V
``` rust
fn eq(&self, other: &V) -> bool {
    self.value.deref() == other
}
```
另一个对象是泛型

#### `src/helper.rs:182:        todo!()` partial_cmp Option<Ordering> 
``` rust
fn partial_cmp(&self, other: &V) -> Option<Ordering> {
    self.value.deref().partial_cmp(other)
}
```
次序比较

### Trait From: convert any to Value
从任意值转换为Value对象， Value对象的value属性类型是Cow

#### `src/helper.rs:191:        todo!()`  v is any type
``` rust
fn from(v: V) -> Self {
    Value::new(Cow::Owned(v))
}
```
从任意类型

#### `src/helper.rs:200:        todo!()`  v type is is Cow
``` rust
fn from(v: Cow<'a, V>) -> Self {
    Value::new(v)
}
```
从Cow类型

#### `src/helper.rs:209:        todo!()`  v type is Value
``` rust
fn from(v: Value<'a, V>) -> Self {
    v.into_inner()
}
```
`into_inner` 


#### `src/helper.rs:218:        todo!()`  v is referenece
``` rust
fn from(v: &V) -> Self {
    Value::new(Cow::Owned(v.clone()))
}
```
引用泛型v

## sled helper function
初始化`sled`数据库，记录数的序列化存储读取
#### `src/helper.rs:228:    todo!()` sled_open initliaze sled instance
``` rust
pub(crate) fn sled_open(path: &str, is_tmp: bool) -> Result<sled::Db> {
    let mut cf = sled::Config::default().path(path).mode(sled::Mode::HighThroughput)
        .use_compression(false);

    if is_tmp {
        cf = cf.temporary(true);
    }
    cf.open().c(d!())
}
```
根据`path` 初始化seld数据库

#### `src/helper.rs:233:    todo!()` read_db_len read counter
``` rust
pub(crate) fn read_db_len(path: &str) -> Result<usize> {
    let bytes = fs::read(path).unwrap();
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&bytes[0..8]);
    Ok(usize::from_le_bytes(buffer))
}
```
从文件读取数据长度

#### `src/helper.rs:238:    todo!()` write_db_len save counter
``` rust
pub(crate) fn write_db_len(path: &str, len: usize) -> Result<()> {
    fs::write(path, usize::to_le_bytes(len)).c(d!())
}
```
序列化存储`len`到文件

## mapx/backend.rs
Mapx基于sled的存储实现

### Trait Iterator
迭代器

#### src/mapx/backend.rs:125:        todo!()   - create iterator
``` rust
pub(super) fn iter(&self) -> MapxIter<K, V> {
    MapxIter {
        iter: self.db.iter(),
        _pd0: PhantomData,
        _pd1: PhantomData,
    }
}
```
返回迭代器

#### src/mapx/backend.rs:180:        todo!()   - iterator next
``` rust
fn next(&mut self) -> Option<Self::Item> {
    self.iter.next()
        .map(|v| v.ok())
        .flatten()
        .map(|(key, value)| {
            (
                pnk!(bincode::deserialize(&key)),
                pnk!(serde_json::from_slice(&value)),
            )
        })
}
```
迭代器`next`实现，写入数据库做了序列化需要反序列化

#### src/mapx/backend.rs:190:        todo!()   - iterator next_back
``` rust
fn next_back(&mut self) -> Option<Self::Item> {
    self.iter
        .next_back()
        .map(|v| v.ok())
        .flatten()
        .map(|(key, value)| {
            (
                pnk!(bincode::deserialize(&key)),
                pnk!(serde_json::from_slice(&value)),
            )
        })
}
```
迭代器`next_back`实现，同样写入数据库做了序列化需要反序列化

### Trait PartialEq
Mapx对象的等值比较运算符重载
- src/mapx/backend.rs:215:        todo!()   - compare is same
``` rust
fn eq(&self, other: &Mapx<K, V>) -> bool {
    !self.iter()
        .zip(other.iter())
        .any(|(a, b)|{ a != b})
}
```
迭代器对比每个值是否都一致

### mapx/mod.rs
Mapx结构的具体实现 
 - in_mem 存储了在内存里的数据
 - in_disk 是backend的Mapx的实例
 - in_mem_cnt 内存数据长度

#### src/mapx/mod.rs:165:        todo!()  - iter consider in mem and disk
``` rust
pub fn iter(&self) -> Box<dyn Iterator<Item = (K, V)> + '_> {
    if self.in_mem.len() == self.in_disk.len() {
        Box::new(MapxIterMem {
            iter: self.in_mem.iter(),
        })
    } else {
        Box::new(MapxIter {
            iter: self.in_disk.iter(),
        })
    }
}
```
如果内存里的数据长度和磁盘数据一致直接返回内存的迭代器

## vecx/backend.rs
Vecx基于sled的存储实现

### Trait Iterator
迭代器相关
#### src/vecx/backend.rs:113:        todo!() create iterator
``` rust
pub(super) fn iter(&self) -> VecxIter<T> {
    VecxIter {
        iter: self.db.iter(),
        _pd: PhantomData
    }
}
```
返回sled数据库迭代器

## vecx/mod.rs
Vecx结构的具体实现 
 - in_mem 存储在内存的数据 BTreeMap
 - in_disk 是backend的Vecx的实例
 - in_mem_cnt 内存数据长度  

### Iterator

#### src/vecx/mod.rs:134:        todo!()  create iterator: should consider in mem and disk
``` rust
pub fn iter(&self) -> Box<dyn Iterator<Item = T> + '_> {
    if self.in_mem.len() == self.in_disk.len() {
        Box::new(VecxIterMem {
            iter: self.in_mem.iter(),
        })
    } else {
        Box::new(VecxIter {
            iter: self.in_disk.iter(),
        })
    }
}
```
和Mapx一样优先从内存

#### src/vecx/mod.rs:166:        todo!() VecxIter next  backend iter
``` rust
fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|v| v.1)
}
```
数据库迭代器`next`实现

#### src/vecx/mod.rs:185:        todo!() VecxIterMem next mem btree iter
``` rust
fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|v| v.1.clone())
}
```
内存迭代器`next`实现

基本上都是迭代器+运算符重载的回顾