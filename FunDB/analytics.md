# funddb
扩展了两种的支持持久化的数据结构
 - Mapx
 - Vecx

提高读取性能，会在内存中缓存一定量的数据，超出的部分需要从从sled取
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