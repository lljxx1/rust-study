# funddb
扩展了两种的支持持久化的数据结构

 - Mapx
 - Vecx

提高写入性能，采取了内存列队机制
当数据大于内存列队限制，会尝试把超出的部分写入sled磁盘数据库
整个代码主要是围绕数据类型的几个操作和运算+迭代器等相关操作

## helper.rs

### Trait Deref Value
- `src/helper.rs:155:        todo!()`

### Trait PartialEq + PartialOrd
- `src/helper.rs:164:        todo!()`
- `src/helper.rs:173:        todo!()`
- `src/helper.rs:182:        todo!()`

### Trait From: convert any to Value
value = Cow<'a, V>
- `src/helper.rs:191:        todo!()`  v is any type
- `src/helper.rs:200:        todo!()`  v type is is Cow
- `src/helper.rs:209:        todo!()`  v type is Value
- `src/helper.rs:218:        todo!()`  v is referenece


### sled helper function
- `src/helper.rs:228:    todo!()` sled_open initliaze sled instance
- `src/helper.rs:233:    todo!()` read_db_len read
- `src/helper.rs:238:    todo!()` write_db_len  write


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
- src/mapx/mod.rs:165:        todo!()  - iter åconsider in mem and disk



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