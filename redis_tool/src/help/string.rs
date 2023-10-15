#[derive(Debug, Clone)]
pub struct StringHelp {}

pub trait Help {
    fn help_set(&self);
    fn help_get(&self);
    fn help_getrange(&self);
    fn help_getset(&self);
    fn help_getbit(&self);
    fn help_mget(&self);
    fn help_setbit(&self);
    fn help_setex(&self);
    fn help_setnx(&self);
    fn help_setrange(&self);
    fn help_strlen(&self);
    fn help_mset(&self);
    fn help_msetnx(&self);
    fn help_psetex(&self);
    fn help_incr(&self);
    fn help_incrby(&self);
    fn help_decr(&self);
    fn help_decrby(&self);
    fn help_append(&self);
}

impl Help for StringHelp {
    fn help_append(&self) {
    println!("
Redis Append 命令用于为指定的 key 追加值。

如果 key 已经存在并且是一个字符串， APPEND 命令将 value 追加到 key 原来的值的末尾。

如果 key 不存在， APPEND 就简单地将给定 key 设为 value ，就像执行 SET key value 一样。

语法
redis Append 命令基本语法如下：

redis 127.0.0.1:6379> APPEND KEY_NAME NEW_VALUE
可用版本
>= 2.0.0

返回值
追加指定值之后， key 中字符串的长度。

实例
127.0.0.1:6379~[db0]#> exists mykey
false
127.0.0.1:6379~[db0]#> append mykey 'hello redis' 
append mykey success (^v^)
+-------+-------------+-----+-----+
| key   | val         | ttl | err |
+-------+-------------+-----+-----+
| mykey | hello redis | -1  | nil |
+-------+-------------+-----+-----+
    ")    
    }

    fn help_decrby(&self) {
        println!("
Redis Decrby 命令将 key 所储存的值减去指定的减量值。

如果 key 不存在，那么 key 的值会先被初始化为 0 ，然后再执行 DECRBY 操作。

如果值包含错误的类型，或字符串类型的值不能表示为数字，那么返回一个错误。

本操作的值限制在 64 位(bit)有符号数字表示之内。

语法
redis Decrby 命令基本语法如下：

redis 127.0.0.1:6379> DECRBY KEY_NAME DECREMENT_AMOUNT
可用版本
>= 1.0.0

返回值
减去指定减量值之后， key 的值。

实例
127.0.0.1:6379~[db0]#> get page   
+------+-----+-----+-----+
| key  | val | ttl | err |
+------+-----+-----+-----+
| page | 19  | -1  | nil |
+------+-----+-----+-----+
127.0.0.1:6379~[db0]#> decrby page 5
(integer) 14
127.0.0.1:6379~[db0]#> get page      
+------+-----+-----+-----+
| key  | val | ttl | err |
+------+-----+-----+-----+
| page | 14  | -1  | nil |
+------+-----+-----+-----+
        
        ")
    }

    fn help_decr(&self) {
    println!("
Redis Decr 命令将 key 中储存的数字值减一。

如果 key 不存在，那么 key 的值会先被初始化为 0 ，然后再执行 DECR 操作。

如果值包含错误的类型，或字符串类型的值不能表示为数字，那么返回一个错误。

本操作的值限制在 64 位(bit)有符号数字表示之内。

语法
redis Decr 命令基本语法如下：

redis 127.0.0.1:6379> DECR KEY_NAME 
可用版本
>= 1.0.0

返回值
执行命令之后 key 的值。

实例
127.0.0.1:6379~[db0]#> set page 20
+------+------+-----+---------+
| walk | key  | val | result  |
+------+------+-----+---------+
| SET  | page | 20  | Success |
+------+------+-----+---------+
127.0.0.1:6379~[db0]#> decr page
(integer) 19
127.0.0.1:6379~[db0]#> get page   
+------+-----+-----+-----+
| key  | val | ttl | err |
+------+-----+-----+-----+
| page | 19  | -1  | nil |
+------+-----+-----+-----+
    ")    
    }

    fn help_incrby(&self) {
        println!("
Redis Incrby 命令将 key 中储存的数字加上指定的增量值。

如果 key 不存在，那么 key 的值会先被初始化为 0 ，然后再执行 INCRBY 命令。

如果值包含错误的类型，或字符串类型的值不能表示为数字，那么返回一个错误。

本操作的值限制在 64 位(bit)有符号数字表示之内。

语法
redis Incrby 命令基本语法如下：

redis 127.0.0.1:6379> INCRBY KEY_NAME INCR_AMOUNT
可用版本
>= 1.0.0

返回值
加上指定的增量值之后， key 的值。

实例
# key 存在且是数字值

127.0.0.1:6379~[db0]#> set page 50
+------+------+-----+---------+
| walk | key  | val | result  |
+------+------+-----+---------+
| SET  | page | 50  | Success |
+------+------+-----+---------+
127.0.0.1:6379~[db0]#> incrby page 20 
(integer) 70
127.0.0.1:6379~[db0]#> get page
+------+-----+-----+-----+
| key  | val | ttl | err |
+------+-----+-----+-----+
| page | 70  | -1  | nil |
+------+-----+-----+-----+

# key 不存在时

127.0.0.1:6379~[db0]#> del page
+------+------+---------+127.0.0.1
+------+------+---------+
| DEL  | page | Success |
+------+------+---------+
127.0.0.1:6379~[db0]#> exists page
false
127.0.0.1:6379~[db0]#> incrby page 20
(integer) 20
127.0.0.1:6379~[db0]#> get page
+------+-----+-----+-----+
| key  | val | ttl | err |
+------+-----+-----+-----+
| page | 20  | -1  | nil |
+------+-----+-----+-----+

# key 不是数字值时

127.0.0.1:6379~[db0]#> del page
+------+------+---------+127.0.0.1
+------+------+---------+
| DEL  | page | Success |
+------+------+---------+
127.0.0.1:6379~[db0]#> set page 'hello' 
+------+------+-------+---------+
| walk | key  | val   | result  |
+------+------+-------+---------+
| SET  | page | hello | Success |
+------+------+-------+---------+
127.0.0.1:6379~[db0]#> incrby page 20
incrby error:An error was signalled by the server: value is not an integer or out of range
        ")
    }

    fn help_incr(&self) {
        println!(
            "
Redis Incr 命令将 key 中储存的数字值增一。

如果 key 不存在，那么 key 的值会先被初始化为 0 ，然后再执行 INCR 操作。

如果值包含错误的类型，或字符串类型的值不能表示为数字，那么返回一个错误。

本操作的值限制在 64 位(bit)有符号数字表示之内。

语法
redis Incr 命令基本语法如下：

redis 127.0.0.1:6379> INCR KEY_NAME 
可用版本
>= 1.0.0

返回值
执行 INCR 命令之后 key 的值。

实例
127.0.0.1:6379~[db0]#> set page 20
+------+------+-----+---------+
| walk | key  | val | result  |
+------+------+-----+---------+
| SET  | page | 20  | Success |
+------+------+-----+---------+
127.0.0.1:6379~[db0]#> incr page
(integer) 21
127.0.0.1:6379~[db0]#> get page   
+------+-----+-----+-----+
| key  | val | ttl | err |
+------+-----+-----+-----+
| page | 21  | -1  | nil |
+------+-----+-----+-----+
        "
        )
    }

    fn help_psetex(&self) {
        println!(
            "
Redis Psetex 命令以毫秒为单位设置 key 的生存时间。

语法
redis Psetex 命令基本语法如下：

redis 127.0.0.1:6379> PSETEX key1 EXPIRY_IN_MILLISECONDS value1 
可用版本
>= 2.6.0

返回值
设置成功时返回 OK 。

实例
127.0.0.1:6379~[db0]#> psetex test5 8000 'test' 
psetex success
+-------+------+-----+-----+
| key   | val  | ttl | err |
+-------+------+-----+-----+
| test5 | test | 8   | nil |
+-------+------+-----+-----+
127.0.0.1:6379~[db0]#> pttl test5
3472 <milliseconds>
127.0.0.1:6379~[db0]#> pttl test5
305 <milliseconds>
        "
        )
    }

    fn help_msetnx(&self) {
        println!(
            "
Redis Msetnx 命令用于所有给定 key 都不存在时，同时设置一个或多个 key-value 对。

语法
redis Msetnx 命令基本语法如下：

redis 127.0.0.1:6379> MSETNX key1 value1 key2 value2 .. keyN valueN 
可用版本
>= 1.0.1

返回值
当所有 key 都成功设置，返回 1 。 如果所有给定 key 都设置失败(至少有一个 key 已经存在)，那么返回 0 。

实例

127.0.0.1:6379~[db0]#> msetnx test1 'hello' test2 'my' test3 'redis' test4 'gorun' 
0
127.0.0.1:6379~[db0]#> keys test*
+--------+-------+
| number | key   |
+--------+-------+
| 1      | test1 |
+--------+-------+
| 2      | test2 |
+--------+-------+
| 3      | test3 |
+--------+-------+
127.0.0.1:6379~[db0]#> msetnx test4 'redis'                          
1
127.0.0.1:6379~[db0]#> keys test*
+--------+-------+
| number | key   |
+--------+-------+
| 1      | test4 |
+--------+-------+
| 2      | test1 |
+--------+-------+
| 3      | test2 |
+--------+-------+
| 4      | test3 |
+--------+-------+
        "
        )
    }

    fn help_mset(&self) {
        println!(
            "
Redis Mset 命令用于同时设置一个或多个 key-value 对。

语法
redis Mset 命令基本语法如下：

redis 127.0.0.1:6379> MSET key1 value1 key2 value2 .. keyN valueN 
可用版本
>= 1.0.1

返回值
总是返回 OK 。

实例
127.0.0.1:6379~[db0]#> mset test1 'hello' test2 'my' test3 'redis' 
OK
127.0.0.1:6379~[db0]#> keys test*
+--------+-------+
| number | key   |
+--------+-------+
| 1      | test1 |
+--------+-------+
| 2      | test2 |
+--------+-------+
| 3      | test3 |
+--------+-------+
    "
        )
    }

    fn help_strlen(&self) {
        println!("
Redis Strlen 命令用于获取指定 key 所储存的字符串值的长度。当 key 储存的不是字符串值时，返回一个错误。

语法
redis Strlen 命令基本语法如下：

redis 127.0.0.1:6379> STRLEN KEY_NAME
可用版本
>= 2.2.0

返回值
字符串值的长度。 当 key 不存在时，返回 0。

实例
127.0.0.1:6379~[db0]#> strlen my_key
+--------+--------+
| key    | length |
+--------+--------+
| my_key | 11     |
+--------+--------+
    ")
    }

    fn help_setrange(&self) {
        println!(
            "
Redis Setrange 命令用指定的字符串覆盖给定 key 所储存的字符串值，覆盖的位置从偏移量 offset 开始。

语法
redis Setrange 命令基本语法如下：

redis 127.0.0.1:6379> SETRANGE KEY_NAME OFFSET VALUE
可用版本
>= 2.2.0

返回值
被修改后的字符串长度。

实例
127.0.0.1:6379~[db0]#> set my_key 'hello world'
+------+--------+-------------+---------+
| walk | key    | val         | result  |
+------+--------+-------------+---------+
| SET  | my_key | hello world | Success |
+------+--------+-------------+---------+
127.0.0.1:6379~[db0]#> setrange my_key 5 'hi'
setrange my_key success, 11
+--------+-------------+-----+-----+
| key    | val         | ttl | err |
+--------+-------------+-----+-----+
| my_key | hellohiorld | -1  | nil |
+--------+-------------+-----+-----+
    "
        )
    }

    fn help_setnx(&self) {
        println!(
            "
### Redis Setnx（SET if Not eXists） 命令在指定的 key 不存在时，为 key 设置指定的值。

### 语法
redis Setnx 命令基本语法如下：

redis 127.0.0.1:6379> SETNX KEY_NAME VALUE
### 可用版本
>= 1.0.0

### 返回值
### 设置成功，返回 1 。 设置失败，返回 0 。

### 实例
127.0.0.1:6379~[db0]#> setnx new_key 'test' 
+------+---------+------+---------+
| walk | key     | val  | result  |
+------+---------+------+---------+
| SET  | new_key | test | Success |
+------+---------+------+---------+
        "
        )
    }

    fn help_setex(&self) {
        println!(
            "
### Redis Setex 命令为指定的 key 设置值及其过期时间。如果 key 已经存在， SETEX 命令将会替换旧的值。

### 语法
### redis Setex 命令基本语法如下：

redis 127.0.0.1:6379> SETEX KEY_NAME TIMEOUT VALUE
### 可用版本
>= 2.0.0

### 返回值
### 设置成功时返回 OK 。

### 实例
127.0.0.1:6379~[db0]#> setex my_key 1000 redis
setex success
+--------+-------+------+-----+
| key    | val   | ttl  | err |
+--------+-------+------+-----+
| my_key | redis | 1000 | nil |
+--------+-------+------+-----+
        "
        )
    }
    fn help_setbit(&self) {
        println!(
            "
### Redis Setbit 命令用于对 key 所储存的字符串值，设置或清除指定偏移量上的位(bit)。

### 语法
### redis Setbit 命令基本语法如下：

redis 127.0.0.1:6379> Setbit KEY_NAME OFFSET
### 可用版本
>= 2.2.0

### 返回值
### 指定偏移量原来储存的位。

### 实例
127.0.0.1:6379~[db0]#> setbit my_key 1 1 
setbit (integer) 1
        "
        )
    }

    fn help_mget(&self) {
        println!("
### Redis Mget 命令返回所有(一个或多个)给定 key 的值。 如果给定的 key 里面，有某个 key 不存在，那么这个 key 返回特殊值 nil 。

### 语法
### redis Mget 命令基本语法如下：

redis 127.0.0.1:6379> MGET KEY1 KEY2 .. KEYN
### 可用版本
>= 1.0.0

### 返回值
### 一个包含所有给定 key 的值的列表。

### 实例
127.0.0.1:6379~[db0]#> mget my_key
+--------+-------------+
| key    | val         |
+--------+-------------+
| my_key | hello redis |
+--------+-------------+
127.0.0.1:6379~[db0]#> mget my_key my_key_1
+----------+-------------+
| key      | val         |
+----------+-------------+
| my_key   | hello redis |
+----------+-------------+
| my_key_1 | hello world |
+----------+-------------+
")
    }

    fn help_getbit(&self) {
        println!(
            "
### Redis Getbit 命令用于对 key 所储存的字符串值，获取指定偏移量上的位(bit)。

### 语法
redis Getbit 命令基本语法如下：

redis 127.0.0.1:6379> GETBIT KEY_NAME OFFSET
### 可用版本
>= 2.2.0

### 返回值
### 字符串值指定偏移量上的位(bit)。

### 当偏移量 OFFSET 比字符串值的长度大，或者 key 不存在时，返回 0 。

### 实例
127.0.0.1:6379~[db0]#> getbit my_key 0
getbit (integer) 0
127.0.0.1:6379~[db0]#> getbit my_key 1
getbit (integer) 1
        "
        )
    }

    fn help_getset(&self) {
        println!(
            "
### Redis Getset 命令用于设置指定 key 的值，并返回 key 的旧值。

### 语法
### redis Getset 命令基本语法如下：

redis 127.0.0.1:6379> GETSET KEY_NAME VALUE
### 可用版本
>= 1.0.0

### 返回值
### 返回给定 key 的旧值。 当 key 没有旧值时，即 key 不存在时，返回 nil 。

### 当 key 存在但不是字符串类型时，返回一个错误。

### 实例
### 首先，设置 mykey 的值并截取字符串。
127.0.0.1:6379~[db0]#> getset my_key  'hello redis' 
+--------+-------------+-----+-----+
| key    | val         | ttl | err |
+--------+-------------+-----+-----+
| my_key | hello world | -1  | nil |
+--------+-------------+-----+-----+
+------+--------+-------------+---------+
| walk | key    | val         | result  |
+------+--------+-------------+---------+
| SET  | my_key | hello redis | Success |
+------+--------+-------------+---------+
        "
        )
    }
    fn help_getrange(&self) {
        println!("
### Redis Getrange 命令用于获取存储在指定 key 中字符串的子字符串。字符串的截取范围由 start 和 end 两个偏移量决定(包括 start 和 end 在内)。

### 语法
### redis Getrange 命令基本语法如下：

redis 127.0.0.1:6379> GETRANGE KEY_NAME start end
### 可用版本
>= 2.4.0

### 返回值
### 截取得到的子字符串。

### 实例
### 首先，设置 mykey 的值并截取字符串
127.0.0.1:6379~[db0]#> getrange my_key 0 3
hell
127.0.0.1:6379~[db0]#> get my_key
+--------+-------------+-----+-----+
| key    | val         | ttl | err |
+--------+-------------+-----+-----+
| my_key | hello world | -1  | nil |
+--------+-------------+-----+-----+
        ")
    }

    fn help_get(&self) {
        println!("
### Redis Get 命令用于获取指定 key 的值。如果 key 不存在，返回 nil 。如果key 储存的值不是字符串类型，返回一个错误。

### 语法
### redis Get 命令基本语法如下：

redis 127.0.0.1:6379> GET KEY_NAME
### 可用版本
>= 1.0.0

### 返回值
### 返回 key 的值，如果 key 不存在时，返回 nil。 如果 key 不是字符串类型，那么返回一个错误。

### 实例
127.0.0.1:6379~[db0]#> set newkey 'hello world' 
+------+--------+-------------+---------+
| walk | key    | val         | result  |
+------+--------+-------------+---------+
| SET  | newkey | hello world | Success |
+------+--------+-------------+---------+")
    }
    fn help_set(&self) {
        println!(
            "
### Redis SET 命令用于设置给定 key 的值。如果 key 已经存储其他值， SET 就覆写旧值，且无视类型。

### 语法
### redis SET 命令基本语法如下：

redis 127.0.0.1:6379> SET KEY_NAME VALUE
### 可用版本
>= 1.0.0

### 返回值
### 在 Redis 2.6.12 以前版本， SET 命令总是返回 OK 。

### 从 Redis 2.6.12 版本开始， SET 在设置操作成功完成时，才返回 OK 。

### 实例
首先，我们在 redis 中创建一个 key 并设置值。
127.0.0.1:6379~[db0]#> set newkey 'hello world' 
+------+--------+-------------+---------+
| walk | key    | val         | result  |
+------+--------+-------------+---------+
| SET  | newkey | hello world | Success |
+------+--------+-------------+---------+
        "
        )
    }
}
