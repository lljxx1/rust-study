## Rust从入门到放弃
> 最近学了大概有10来天rust了，已经手抄了好几个小项目的代码。跃跃欲试就给自己找了几道题目玩玩，过程还挺好玩的hh

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

按照我浅薄的理解，`hange_value`里面是无法访问`main`块里作用域的，如果不把`i`传参进去，
要想把`i`改为1是不太可能的。如果只能在`change_value`里加逻辑的话理论上是做不到覆盖`i`的值。如果可以修改外面的代码话`let mut = i; change_value(&mut i);`即可。

这里复习了很多作用域的定义，还google了很多可能可以的办法，结论应该是不行... 但回到题目的要求：可以让这个程序正常运行，突然想到前几天写`minigrep`的时候用到的中断`process::exit`，既然`i`值理论上无法修改，但我可以在这个函数块里让进程停止运行，这样一来后面的`assert_eq`也就不会继续运行了hhh

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


# funddb
第二道大题是个结构化的项目了里面有很多实现被抹去了，需要自己完善细节，但实际的话这个项目相似度很高很多逻辑是重复相似的仔细看，很多实现都能在里面找到解

## 项目分析
扩展了两种的支持持久化的数据结构
 - Mapx
 - Vecx

提高读取性能，会在内存中缓存一定量的数据，超出的部分需要从从`sled`取  
整个代码主要是围绕实现这两个数据类型的，以及各种操作和运算+迭代器的Trait实现等相关操作

## helper.rs

### Trait Deref Value
- `src/helper.rs:155:        todo!()` return reference

### Trait PartialEq + PartialOrd
- `src/helper.rs:164:        todo!()` other type is Value
- `src/helper.rs:173:        todo!()` other is reference V
- `src/helper.rs:182:        todo!()` partial_cmp Option<Ordering> 

### Trait From: convert any to Value
value = Cow<'a, V>
- `src/helper.rs:191:        todo!()`  v is any type
- `src/helper.rs:200:        todo!()`  v type is is Cow
- `src/helper.rs:209:        todo!()`  v type is Value
- `src/helper.rs:218:        todo!()`  v is referenece


### sled helper function
sled initliaze, data counter
- `src/helper.rs:228:    todo!()` sled_open initliaze sled instance
- `src/helper.rs:233:    todo!()` read_db_len read counter
- `src/helper.rs:238:    todo!()` write_db_len save counter


## mapx/backend.rs
Mapx基于sled的存储实现

### Trait Iterator
- src/mapx/backend.rs:125:        todo!()   - create iterator
- src/mapx/backend.rs:180:        todo!()   - iterator next
- src/mapx/backend.rs:190:        todo!()   - iterator next_back

### Trait PartialEq
- src/mapx/backend.rs:215:        todo!()   - compare is same

### mapx/mod.rs
Mapx结构的具体实现 
 - in_mem 存储了在内存里的数据
 - in_disk 是backend的Mapx的实例
 - in_mem_cnt 内存数据长度

Trait 
- src/mapx/mod.rs:165:        todo!()  - iter consider in mem and disk


## mapx/backend.rs
Mapx基于sled的存储实现

### Trait Iterator
- src/mapx/backend.rs:125:        todo!()   - create iterator
- src/mapx/backend.rs:180:        todo!()   - iterator next
- src/mapx/backend.rs:190:        todo!()   - iterator next_back

### Trait PartialEq
- src/mapx/backend.rs:215:        todo!()   - compare is same

## mapx/mod.rs
Mapx结构的具体实现 
 - in_mem 存储在内存的数据 HashMap
 - in_disk 是backend的Mapx的实例
 - in_mem_cnt 内存数据长度

Trait 
- src/mapx/mod.rs:165:        todo!()  - iter consider in mem and disk


## vecx/backend.rs
Vecx基于sled的存储实现

### Trait Iterator
- src/vecx/backend.rs:113:        todo!() create iterator

## vecx/mod.rs
Vecx结构的具体实现 
 - in_mem 存储在内存的数据 BTreeMap
 - in_disk 是backend的Vecx的实例
 - in_mem_cnt 内存数据长度  

### Trait Iterator +
- src/vecx/mod.rs:134:        todo!()  create iterator: should consider in mem and disk
- src/vecx/mod.rs:166:        todo!() VecxIter next  backend iter
- src/vecx/mod.rs:185:        todo!() VecxIterMem next mem btree iter