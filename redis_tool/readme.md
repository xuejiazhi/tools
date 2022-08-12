# redis_tool 简价
[![imi License](https://img.shields.io/badge/license-AGPLv3-brightgreen.svg)](https://github.com/xuejiazhi/tools/blob/master/redis_tool/LICENSE)

redis_tool 是一个模拟redis-cli的工具,对redis查询出来的结果进行格式化的展示,采用rust进行开发,可以多平台下面编译使用：

- **跨平台**： 可以在多平台下编译，跨平台使用；

- **零学习成本**：完全兼容redis命令,也可以做为redis命令的学习工具；

- **互动 Console**: 通过命令行 console。 


### 登录
**cmd**: 
```bash
    redis_tool <host> <port> [password]
```
 
**Case**:
```bash
  root~# redis_tool.exe 127.0.0.1 6379 "password"
    .-"""-.
    / .===. \
    \/ 6 6 \/
    ( \___/ )
 _________ooo__\_____/_____________
/                                  \
|    Connect Redis Success!         |
\_______________________ooo________/
     |  |  |
     |_ | _|
     |  |  |
     |__|__|
     /-'Y'-\
    (__/ \__)
127.0.0.1:6379~[db0]#>
```

#  Keys
## Type 命令用于返回 key 所储存的值的类型

```bash
语法
redis Renamenx 命令基本语法如下：

redis 127.0.0.1:6379> TYPE KEY_NAME 
可用版本
>= 1.0.0

返回值
返回 key 的数据类型，数据类型有：

none (key不存在)
string (字符串)
list (列表)
set (集合)
zset (有序集)
hash (哈希表)

实例
127.0.0.1 :6379~[db0]#> type my_key
+--------+--------+
| key    | type   |        
+--------+--------+        
| my_key | string |        
+--------+--------+
```
## DUMP 命令用于序列化给定 key ,并返回被序列化的值
```bash
语法
redis DUMP 命令基本语法如下：

    redis 127.0.0.1:6379> DUMP KEY_NAME
可用版本
    >= 2.6.0

返回值
    如果 key 不存在，那么返回 nil 。 否则，返回序列化之后的值。

实例
    首先，我们在 redis 中创建一个 key 并设置值。

    redis> SET greeting 'hello, dumping world!'
    OK
现在使用 DUMP 序列化键值。

redis> DUMP greeting
\\x00\\x15hello, dumping world!\\x06\\x00E\\xa0Z\\x82\\xd8r\\xc1\\xde

redis> DUMP not-exists-key
(nil)
```

## EXISTS 命令用于检查给定 key 是否存在
```bash
## Redis EXISTS 命令用于检查给定 key 是否存在。

## 语法
## redis EXISTS 命令基本语法如下：

redis 127.0.0.1:6379> EXISTS KEY_NAME
## 可用版本
>= 1.0.0

## 返回值
## 若 key 存在返回 1 ，否则返回 0 。

## 实例
redis 127.0.0.1:6379> EXISTS test-key
(integer) 0
## 现在我们创建一个名为test-key 的键并赋值，再使用 EXISTS 命令。

redis 127.0.0.1:6379> set test-key newkey
OK
redis 127.0.0.1:6379> EXISTS test-key
(integer) 1
```

## Expire 命令用于设置 key 的过期时间，key 过期后将不再可用。单位以秒计
```bash
## Redis Expire 命令用于设置 key 的过期时间，key 过期后将不再可用。单位以秒计。

## 语法
## redis Expire 命令基本语法如下：

## redis 127.0.0.1:6379> Expire KEY_NAME TIME_IN_SECONDS
## 可用版本
>= 1.0.0

## 返回值
## 设置成功返回 1 。 当 key 不存在或者不能为 key 设置过期时间时(比如在低于 2.1.3 版本的 Redis 中你尝试更新 key 的过期时间)返回 0 。

## 实例
## 首先创建一个 key 并赋值：

redis 127.0.0.1:6379> SET runooobkey redis
OK
## 为 key 设置过期时间：

redis 127.0.0.1:6379> EXPIRE runooobkey 60
(integer) 1
以上实例中我们为键 runooobkey 设置了过期时间为 1 分钟，1分钟后该键会自动删除。
```

## Expireat 命令用于以 UNIX 时间戳(unix timestamp)格式设置 key 的过期时间。key 过期后将不再可用
```bash
## Redis Expireat 命令用于以 UNIX 时间戳(unix timestamp)格式设置 key 的过期时间。key 过期后将不再可用。

## 语法
## redis Expireat 命令基本语法如下：

redis 127.0.0.1:6379> Expireat KEY_NAME TIME_IN_UNIX_TIMESTAMP
## 可用版本
>= 1.0.0

## 返回值
## 设置成功返回 1 。 当 key 不存在或者不能为 key 设置过期时间时(比如在低于 2.1.3 版本的 Redis 中你尝试更新 key 的过期时间)返回 0 。

## 实例
## 首先创建一个 key 并赋值：
    
127.0.0.1:6379~[db0]#> SET mykey `Hello`
+------+-------+---------+---------+
| walk | key   | val     | result  |
+------+-------+---------+---------+
| SET  | mykey | `Hello` | Success |
+------+-------+---------+---------+
## 为 key 设置过期时间：
    
127.0.0.1:6379~[db0]#> expireat mykey 1669356402000
Expireat success
+-------+-------+------------+-----+
| key   | val   | ttl        | err |
+-------+-------+------------+-----+
| mykey | hello | 1249705899 | nil |
+-------+-------+------------+-----+
```

## PEXPIRE 命令和 EXPIRE 命令的作用类似，但是它以毫秒为单位设置 key 的生存时间，而不像 EXPIRE 命令那样，以秒为单位
```bash
## Redis PEXPIRE 命令和 EXPIRE 命令的作用类似，但是它以毫秒为单位设置 key 的生存时间，而不像 EXPIRE 命令那样，以秒为单位。
## 语法
##  redis PEXPIRE 命令基本语法如下：

PEXPIRE key milliseconds
##  可用版本
>= 2.6.0

##  返回值
## 设置成功，返回 1

## key 不存在或设置失败，返回 0

## 实例
## 首先创建一个 key 并赋值：

redis> SET mykey `Hello`
+------+-------+---------+---------+
| walk | key   | val     | result  |
+------+-------+---------+---------+
| SET  | mykey | `Hello` | Success |
+------+-------+---------+---------+
redis> PEXPIRE mykey 1500
PExpire success
+-------+---------+-----+-----+
| key   | val     | ttl | err |
+-------+---------+-----+-----+
| mykey | `Hello` | 1   | nil |
+-------+---------+-----+-----+
```

## PEXPIREAT 命令用于设置 key 的过期时间，以毫秒计。key 过期后将不再可用
```bash
## Redis PEXPIREAT 命令用于设置 key 的过期时间，以毫秒计。key 过期后将不再可用。

## 语法
## redis PEXPIREAT 命令基本语法如下：

redis 127.0.0.1:6379> PEXPIREAT KEY_NAME TIME_IN_MILLISECONDS_IN_UNIX_TIMESTAMP
## 可用版本
>= 1.0.0

## 返回值
## 设置成功返回 1 。 当 key 不存在或者不能为 key 设置过期时间时(比如在低于 2.1.3 版本的 Redis 中你尝试更新 key 的过期时间)返回 0 。

## 实例
## 首先创建一个 key 并赋值：

127.0.0.1:6379[db0]#>  SET mykey Hello
redis> SET mykey `Hello`
+------+-------+---------+---------+
| walk | key   | val     | result  |
+------+-------+---------+---------+
| SET  | mykey | Hello   | Success |
+------+-------+---------+---------+
## 为 key 设置过期时间：

127.0.0.1:6379[db0]#> pexpireat mykey 1669356402000
PExpireat success
+-------+-------+---------+-----+
| key   | val   | ttl     | err |
+-------+-------+---------+-----+
| mykey | Hello | 9970754 | nil |
+-------+-------+---------+-----+
```

## KEYS查找所有符合给定模式( pattern)的 key
```bash
## 语法
## redis KEYS 命令基本语法如下：

redis 127.0.0.1:6379> KEYS PATTERN
## 可用版本
>= 1.0.0

##  返回值
### 符合给定模式的 key 列表 (Array)。

## 实例
### 查找以 b 为开头的 key：

127.0.0.1:6379~[db0]#> keys b* 
+--------+-----+
| number | key |
+--------+-----+
| 1      | byz |
+--------+-----+
### 获取 redis 中所有的 key 可用使用 *。

127.0.0.1:6379~[db0]#> keys *
+--------+-------+
| number | key   |
+--------+-------+
| 1          | lx       |
+--------+-------+
| 2         | byz   |
+--------+-------+
| 3          | mykey |
+--------+-------+
```

## DEL 命令用于删除已存在的键。不存在的 key 会被忽略
```bash
## 语法
  redis DEL 命令基本语法如下：

  redis 127.0.0.1:6379> DEL KEY_NAME
## 可用版本
  >= 1.0.0

## 返回值
  被删除 key 的数量。

## 实例
  首先，我们在 redis 中创建一个 key 并设置值。

  127.0.0.1:6379~[db0]#> set testKey hello
    +------+---------+-------+---------+
    | walk | key     | val   | result  |
    +------+---------+-------+---------+
    | SET  | testKey | hello | Success |
    +------+---------+-------+---------+
## 现在我们删除已创建的 key。
  127.0.0.1:6379~[db0]#> del testKey
    +------+---------+---------+
    | walk | key     | result  |
    +------+---------+---------+
    | DEL  | testKey | Success |
    +------+---------+---------+
```

## MOVE 命令用于将当前数据库的 key 移动到给定的数据库 db 当中。
```bash
### 语法
### redis Move 命令基本语法如下：

127.0.0.1:6379~[db0]#> MOVE KEY_NAME DESTINATION_DATABASE

### 可用版本
>= 1.0.0

## 在db0下查看所有的key
127.0.0.1:6379~[db0]#> keys *
+--------+-------+
| number | key   |
+--------+-------+
| 1      | lx    |
+--------+-------+
| 2      | byz   |
+--------+-------+
| 3      | mykey |
+--------+-------+

## 移动key 为lx 到 db1
127.0.0.1:6379~[db0]#> move lx 1
Move Key lx success!

## 切换到db1
127.0.0.1:6379~[db0]#> db1
switch db 1 Success!

## 查看db1下的key 可以看到lx已移过来了
127.0.0.1:6379~[db1]#> keys *
+--------+-----+
| number | key |
+--------+-----+
| 1      | lx  |
+--------+-----+
| 2      | x   |
+--------+-----+ 
```    

## PERSIST 命令用于移除给定 key 的过期时间，使得 key 永不过期。
```bash
## 语法
redis PERSIST 命令基本语法如下：

redis 127.0.0.1:6379> persist KEY_NAME
## 可用版本
>= 2.2.0

## 操作
127.0.0.1:6379~[db0]#> get mykey
+-------+-------+---------+-----+
| key   | val   | ttl     | err |
+-------+-------+---------+-----+
| mykey | hello | 9810959 | nil |
+-------+-------+---------+-----+
127.0.0.1:6379~[db0]#> persist mykey
Persist Key mykey Success
+-------+-------+-----+-----+
| key   | val   | ttl | err |
+-------+-------+-----+-----+
| mykey | hello | -1  | nil |
+-------+-------+-----+-----+
```     

## pttl命令以毫秒为单位返回 key 的剩余过期时间。
```bash
    语法
    redis pttl命令基本语法如下：
    
    redis 127.0.0.1:6379> pttlKEY_NAME
    可用版本
    >= 2.6.0
    
    返回值
    当 key 不存在时，返回 -2 。 当 key 存在但没有设置剩余生存时间时，返回 -1 。 否则，以毫秒为单位，返回 key 的剩余生存时间。
    
    注意：在 Redis 2.8 以前，当 key 不存在，或者 key 没有设置剩余生存时间时，命令都返回 -1 。
    
    127.0.0.1:6379~[db0]#> keys *
    +--------+-------+
    | number | key   |
    +--------+-------+
    | 1      | byz   |
    +--------+-------+
    | 2      | mykey |
    +--------+-------+
    127.0.0.1:6379~[db0]#> pttlmykey
    -1 <milliseconds>
```     

## Redis TTL 命令以秒为单位返回 key 的剩余过期时间
```bash
### 语法
redis TTL 命令基本语法如下：

redis 127.0.0.1:6379> TTL KEY_NAME
### 可用版本
>= 1.0.0

### 返回值
### 当 key 不存在时，返回 -2 。 当 key 存在但没有设置剩余生存时间时，返回 -1 。 否则，以秒为单位，返回 key 的剩余生存时间。

### 注意：在 Redis 2.8 以前，当 key 不存在，或者 key 没有设置剩余生存时间时，命令都返回 -1 。

实例

127.0.0.1:6379~[db0]#> keys *
+--------+-------+
| number | key   |
+--------+-------+
| 1      | byz   |
+--------+-------+
| 2      | mykey |
+--------+-------+
127.0.0.1:6379~[db0]#> ttl mykey
-1 <seconds>
```     

## RANDOMKEY 命令从当前数据库中随机返回一个 key
```bash
### 语法
### redis RANDOMKEY 命令基本语法如下：

redis 127.0.0.1:6379> RANDOMKEY 
### 可用版本
>= 1.0.0

### 返回值
### 当数据库不为空时，返回一个 key 。 当数据库为空时，返回 nil （windows 系统返回 null）。

实例
127.0.0.1:6379~[db0]#> randomkey 
+-----------+------+-----+
| randomkey | type | ttl |
+-----------+------+-----+
| byz       | hash | -1  |
+-----------+------+-----+
127.0.0.1:6379~[db0]#> randomkey
+-----------+--------+-----+
| randomkey | type   | ttl |
+-----------+--------+-----+
| mykey     | string | -1  |
+-----------+--------+-----+
```     

## Rename 命令用于修改 key 的名称 
```bash
### 语法
### redis Rename 命令基本语法如下：

redis 127.0.0.1:6379> RENAME OLD_KEY_NAME NEW_KEY_NAME
### 可用版本
>= 1.0.0

### 返回值
### 改名成功时提示 OK ，失败时候返回一个错误。

当 OLD_KEY_NAME 和 NEW_KEY_NAME 相同，或者 OLD_KEY_NAME 不存在时，返回一个错误。 当 NEW_KEY_NAME 已经存在时， RENAME 命令将覆盖旧值。

实例
127.0.0.1:6379~[db0]#> rename mykey my_key
RENAME Success Old key <mykey>, New Key <my_key>
+--------+-------+-----+-----+
| key       | val      | ttl     | err   |
+--------+-------+-----+-----+
| my_key | hello   | -1     | nil    |
+--------+-------+-----+-----+
```     

## Renamenx 命令用于在新的 key 不存在时修改 key 的名称 
```bash
### 语法
### redis Renamenx 命令基本语法如下：

### redis 127.0.0.1:6379> RENAMENX OLD_KEY_NAME NEW_KEY_NAME
### 可用版本
>= 1.0.0

### 返回值
### 修改成功时，返回 1 。 如果 NEW_KEY_NAME 已经存在，返回 0 。

### 实例

127.0.0.1:6379~[db0]#> renamenx mykey my_key
RENAME Success Old key <mykey>, New Key <my_key>
+--------+-------+-----+-----+
| key       | val      | ttl     | err   |
+--------+-------+-----+-----+
| my_key | hello   | -1     | nil    |
+--------+-------+-----+-----+
```     


## Scan 命令用于迭代数据库中的数据库键
```bash
SCAN 命令是一个基于游标的迭代器，每次被调用之后， 都会向用户返回一个新的游标， 用户在下次迭代时需要使用这个新游标作为 SCAN 命令的游标参数， 以此来延续之前的迭代过程。

###语法
redis Scan 命令基本语法如下：

SCAN cursor [MATCH pattern] [COUNT count]
cursor - 游标。
pattern - 匹配的模式。
count - 指定从数据集里返回多少元素，默认值为 10 。
###可用版本
>= 2.8.0

### 实例

10.161.55.194:6379~[db0]#> scan 2
Cursor -> 0
+--------+--------+
| number | key    |        
+--------+--------+        
| 1      | my_key |        
+--------+--------+        
10.161.55.194:6379~[db0]#> scan 0
Cursor -> 0
+--------+--------+
| number | key    |
+--------+--------+
| 1      | byz    |
+--------+--------+
| 2      | my_key |
+--------+--------+
```     

##  Append 命令用于为指定的 key 追加值。
```bash
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
``` 


##  Decrby 命令将 key 所储存的值减去指定的减量值。
```bash
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
``` 

##  Decr 命令将 key 中储存的数字值减一。

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
```

##  Incrby 命令将 key 中储存的数字加上指定的增量值。
```bash
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
``` 

##  Incr 命令将 key 中储存的数字值增一
```bash
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
```

##  Psetex 命令以毫秒为单位设置 key 的生存时间。
```bash
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
``` 


##  Msetnx 命令用于所有给定 key 都不存在时，同时设置一个或多个 key-value 对。
```bash
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
``` 

##  Mset 命令用于同时设置一个或多个 key-value 对。
```bash
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
``` 


##  Strlen 命令用于获取指定 key 所储存的字符串值的长度。当 key 储存的不是字符串值时，返回一个错误。
```bash
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
``` 


##  Setrange 命令用指定的字符串覆盖给定 key 所储存的字符串值，覆盖的位置从偏移量 offset 开始。
```bash
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
```


##  Setnx（SET if Not eXists） 命令在指定的 key 不存在时，为 key 设置指定的值。
```bash
语法
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
```


##  Setex 命令为指定的 key 设置值及其过期时间。如果 key 已经存在， SETEX 命令将会替换旧的值。
```bash
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
```


##  Setbit 命令用于对 key 所储存的字符串值，设置或清除指定偏移量上的位(bit)。
```bash
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
```


##  Mget 命令返回所有(一个或多个)给定 key 的值。 如果给定的 key 里面，有某个 key 不存在，那么这个 key 返回特殊值 nil 。
```bash
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
``` 


##  Getbit 命令用于对 key 所储存的字符串值，获取指定偏移量上的位(bit)。
```bash
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
``` 


##  Getset 命令用于设置指定 key 的值，并返回 key 的旧值。
```bash
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
``` 

##  Getrange 命令用于获取存储在指定 key 中字符串的子字符串。字符串的截取范围由 start 和 end 两个偏移量决定(包括 start 和 end 在内)。
```bash
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
``` 

##  Get 命令用于获取指定 key 的值。如果 key 不存在，返回 nil 。如果key 储存的值不是字符串类型，返回一个错误。
```bash
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
+------+--------+-------------+---------+"
```

##  SET 命令用于设置给定 key 的值。如果 key 已经存储其他值， SET 就覆写旧值，且无视类型。
```bash
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
 ```
