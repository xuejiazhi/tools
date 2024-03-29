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

#  String
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

#  Hash
```bash
## HSCAN 命令用于迭代哈希表中的键值对。

语法
redis HSCAN 命令基本语法如下：

HSCAN key cursor [MATCH pattern] [COUNT count]
cursor - 游标。
pattern - 匹配的模式。
count - 指定从数据集里返回多少元素，默认值为 10 。
可用版本
>= 2.8.0

返回值
返回的每个元素都是一个元组，每一个元组元素由一个字段(field) 和值（value）组成。

实例
127.0.0.1:6379~[db0]#> hset hashset a hello
hset hash key hashset field a success (^v^)
127.0.0.1:6379~[db0]#> hset hashset b world 
hset hash key hashset field b success (^v^)
127.0.0.1:6379~[db0]#> hgetall hashset
+-------+-------+
| field | value |
+-------+-------+
| b     | world |
+-------+-------+
| a     | hello |
+-------+-------+
127.0.0.1:6379~[db0]#> hscan hashset 0 match *
+--------+-------------+
| number | hscan-value |
+--------+-------------+
| 0      | a           |
+--------+-------------+
| 1      | hello       |
+--------+-------------+
| 2      | b           |
+--------+-------------+
| 3      | world       |
+--------+-------------+
```


## Hvals 命令返回哈希表所有的值。
```bash
语法
redis Hvals 命令基本语法如下：

redis 127.0.0.1:6379> HVALS key
可用版本
>= 2.0.0

返回值
一个包含哈希表中所有值的列表。 当 key 不存在时，返回一个空表。

实例
127.0.0.1:6379~[db0]#> hvals mykey
+--------+-------+
| number | field |
+--------+-------+
| 1      | 10.5  |
+--------+-------+
| 2      | 11.5  |
+--------+-------+
| 3      | 12.3  |
+--------+-------+
| 4      | 23.1  |
+--------+-------+
| 5      | 11.1  |
+--------+-------+
```



## Hsetnx 命令用于为哈希表中不存在的的字段赋值 。
```bash
如果哈希表不存在，一个新的哈希表被创建并进行 HSET 操作。

如果字段已经存在于哈希表中，操作无效。

如果 key 不存在，一个新哈希表被创建并执行 HSETNX 命令。

语法
redis Hsetnx 命令基本语法如下：

redis 127.0.0.1:6379> HSETNX KEY_NAME FIELD VALUE
可用版本
>= 2.0.0

返回值
设置成功，返回 1 。 如果给定字段已经存在且没有操作被执行，返回原来的值，不会被设置。

实例
127.0.0.1:6379~[db0]#> hsetnx nxkey f 11.4
hsetnx field f success
+----------+-------+-------+
| hash-key | field | value |
+----------+-------+-------+
| nxkey    | f     | 11.4  |
+----------+-------+-------+
127.0.0.1:6379~[db0]#> hsetnx nxkey f 11.5
hsetnx field f success
+----------+-------+-------+
| hash-key | field | value |
+----------+-------+-------+
| nxkey    | f     | 11.4  |
+----------+-------+-------+   
```



## Hset 命令用于为哈希表中的字段赋值 。
```bash
如果哈希表不存在，一个新的哈希表被创建并进行 HSET 操作。

如果字段已经存在于哈希表中，旧值将被覆盖。

语法
redis Hset 命令基本语法如下：

redis 127.0.0.1:6379> HSET KEY_NAME FIELD VALUE 
可用版本
>= 2.0.0

返回值
如果字段是哈希表中的一个新建字段，并且值设置成功，返回 1 。 如果哈希表中域字段已经存在且旧值已被新值覆盖，返回 0 。

实例
127.0.0.1:6379~[db0]#> hset mykey d 11.1 
hset hash key mykey field d success (^v^)
127.0.0.1:6379~[db0]#> hgetall mykey     
+-------+-------+
| field | value |
+-------+-------+
| a     | 11.5  |
+-------+-------+
| f     | 10.5  |
+-------+-------+
| d     | 11.1  |
+-------+-------+
| b     | 12.3  |
+-------+-------+
| c     | 23.1  |
+-------+-------+
```
	
	

## Hmset 命令用于同时将多个 field-value (字段-值)对设置到哈希表中。
```bash
此命令会覆盖哈希表中已存在的字段。

如果哈希表不存在，会创建一个空哈希表，并执行 HMSET 操作。

语法
redis Hmset 命令基本语法如下：

redis 127.0.0.1:6379> HMSET KEY_NAME FIELD1 VALUE1 ...FIELDN VALUEN  
可用版本
>= 2.0.0

返回值
如果命令执行成功，返回 OK 。

实例
127.0.0.1:6379~[db0]#> hmset mykey b 12.3 c 23.1
hmset OK
127.0.0.1:6379~[db0]#> hgetall mykey
+-------+-------+
| field | value |
+-------+-------+
| a     | 11.5  |
+-------+-------+
| b     | 12.3  |
+-------+-------+
| f     | 10.5  |
+-------+-------+
| c     | 23.1  |
+-------+-------+
```
 
 
 

## Hmget 命令用于返回哈希表中，一个或多个给定字段的值。
```bash
如果指定的字段不存在于哈希表，那么返回一个 nil 值。

语法
redis Hmget 命令基本语法如下：

redis 127.0.0.1:6379> HMGET KEY_NAME FIELD1...FIELDN 
可用版本
>= 2.0.0

返回值
一个包含多个给定字段关联值的表，表值的排列顺序和指定字段的请求顺序一样。

实例
127.0.0.1:6379~[db0]#> hmget mykey f a
+-----+------+
| key | val  |
+-----+------+
| a   | 11.5 |
+-----+------+
| f   | 10.5 |
+-----+------+
```   
   
   
   

## Hlen 命令用于获取哈希表中字段的数量。
```bash
语法
redis Hlen 命令基本语法如下：

redis 127.0.0.1:6379> HLEN KEY_NAME 
可用版本
>= 2.0.0

返回值
哈希表中字段的数量。 当 key 不存在时，返回 0 。

实例
127.0.0.1:6379~[db0]#> hlen mykey
hlen hash key (mykey) field length is 2 !
```
 
 
 
## Hkeys 命令用于获取哈希表中的所有域（field）。

语法
redis Hkeys 命令基本语法如下：

redis 127.0.0.1:6379> HKEYS key 
可用版本
>= 2.0.0

返回值
包含哈希表中所有域（field）列表。 当 key 不存在时，返回一个空列表。

实例
127.0.0.1:6379~[db0]#> hkeys mykey       
+--------+-------+
| number | field |
+--------+-------+
| 1      | f     |
+--------+-------+
| 2      | a     |
+--------+-------+  
```
 
 
 
## Hincrbyfloat 命令用于为哈希表中的字段值加上指定浮点数增量值。
```bash
如果指定的字段不存在，那么在执行命令前，字段的值被初始化为 0 。

语法
redis Hincrbyfloat 命令基本语法如下：

HINCRBYFLOAT key field increment
可用版本
>= 2.6.0

返回值
执行 Hincrbyfloat 命令之后，哈希表中字段的值。

实例

127.0.0.1:6379~[db0]#> hset mykey f 10.5
hset hash key mykey field f success (^v^)
127.0.0.1:6379~[db0]#> hincrbyfloat mykey f 0.1
hincrbyfloat key mykey field f success!
+----------+-------+-------+
| hash-key | field | value |
+----------+-------+-------+
| mykey    | f     | 10.6  |
+----------+-------+-------+
127.0.0.1:6379~[db0]#> hincrbyfloat mykey f -0.1 
hincrbyfloat key mykey field f success!
+----------+-------+-------+
| hash-key | field | value |
+----------+-------+-------+
| mykey    | f     | 10.5  |
+----------+-------+-------+
```





## Hincrby 命令用于为哈希表中的字段值加上指定增量值。
```bash
增量也可以为负数，相当于对指定字段进行减法操作。

如果哈希表的 key 不存在，一个新的哈希表被创建并执行 HINCRBY 命令。

如果指定的字段不存在，那么在执行命令前，字段的值被初始化为 0 。

对一个储存字符串值的字段执行 HINCRBY 命令将造成一个错误。

本操作的值被限制在 64 位(bit)有符号数字表示之内。

语法
redis Hincrby 命令基本语法如下：

redis 127.0.0.1:6379> HINCRBY KEY_NAME FIELD_NAME INCR_BY_NUMBER 
可用版本
>= 2.0.0

返回值
执行 HINCRBY 命令之后，哈希表中字段的值。

实例
127.0.0.1:6379~[db0]#> hset myhash 5
HSET  Cli length is failed,length is  4
127.0.0.1:6379~[db0]#> hset myhash f 5 
hset hash key myhash field f success (^v^)
127.0.0.1:6379~[db0]#> hgetall myhash
+-------+-------+
| field | value |
+-------+-------+
| f     | 5     |
+-------+-------+
127.0.0.1:6379~[db0]#> hincrby myhash f 1 
hincrby key myhash field f success!
+----------+-------+-------+
| hash-key | field | value |
+----------+-------+-------+
| myhash   | f     | 6     |
+----------+-------+-------+
```  
	  
	   
	  
## Hgetall 命令用于返回哈希表中，所有的字段和值。
```bash
在返回值里，紧跟每个字段名(field name)之后是字段的值(value)，所以返回值的长度是哈希表大小的两倍。

语法
redis Hgetall 命令基本语法如下：

redis 127.0.0.1:6379> HGETALL KEY_NAME 
可用版本
>= 2.0.0

返回值
以列表形式返回哈希表的字段及字段值。 若 key 不存在，返回空列表。

实例
127.0.0.1:6379~[db0]#> hgetall byz
+-------+-----------------------+
| field | value                 |
+-------+-----------------------+
| lbs   | ['name':'luobaishun'] |
+-------+-----------------------+
| xjz   | ['name':'xuejiazhi']  |
+-------+-----------------------+
| lx    | ['name':'liuxiang']   |
+-------+-----------------------+
```  
   
   
   
   

## Hget 命令用于返回哈希表中指定字段的值。
```bash
语法
redis Hget 命令基本语法如下：

redis 127.0.0.1:6379> HGET KEY_NAME FIELD_NAME 
可用版本
>= 2.0.0

返回值
返回给定字段的值。如果给定的字段或 key 不存在时，返回 nil 。

实例
127.0.0.1:6379~[db0]#> hget byz lx
+----------+-------+---------------------+
| hash-key | field | value               |
+----------+-------+---------------------+
| byz      | lx    | ['name':'liuxiang'] |
+----------+-------+---------------------+
```   
   
   
   

## Hexists 命令用于查看哈希表的指定字段是否存在。
```bash
语法
redis Hexists 命令基本语法如下：

redis 127.0.0.1:6379> HEXISTS KEY_NAME FIELD_NAME 
可用版本
>= 2.0.0

返回值
如果哈希表含有给定字段，返回 1 。 如果哈希表不含有给定字段，或 key 不存在，返回 0 。

实例
127.0.0.1:6379~[db0]#> hexists byz lx
hexists hash key byz field lx exists!
+----------+-------+---------------------+
| hash-key | field | value               |
+----------+-------+---------------------+
| byz      | lx    | ['name':'liuxiang'] |
+----------+-------+---------------------+
```
	  
	  
	  
	  
## Hdel 命令用于删除哈希表 key 中的一个或多个指定字段，不存在的字段将被忽略。
```bash
语法
redis Hdel 命令基本语法如下：

redis 127.0.0.1:6379> HDEL KEY_NAME FIELD1.. FIELDN 
可用版本
>= 2.0.0

返回值
被成功删除字段的数量，不包括被忽略的字段。

实例
127.0.0.1:6379~[db0]#> hdel byz hdf
hdel hash key byz field hdf success (^v^)
127.0.0.1:6379~[db0]#> hget byz
HGET  Cli length is failed,length is  3
127.0.0.1:6379~[db0]#> hgetall byz 
+-------+-----------------------+
| field | value                 |
+-------+-----------------------+
| xjz   | ['name':'xuejiazhi']  |
+-------+-----------------------+
| lbs   | ['name':'luobaishun'] |
+-------+-----------------------+
| lx    | ['name':'liuxiang']   |
+-------+-----------------------+
```


# List
## Rpushx 命令用于将一个值插入到已存在的列表尾部(最右边)。如果列表不存在，操作无效。
```bash
语法
redis Rpushx 命令基本语法如下：

redis 127.0.0.1:6379> RPUSHX KEY_NAME VALUE1..VALUEN
可用版本
>= 2.2.0

返回值
执行 Rpushx 操作后，列表的长度。

实例
127.0.0.1:6379~[db0]#> keys mylist
+--------+-----+
| number | key |
+--------+-----+
127.0.0.1:6379~[db0]#> rpushx mylist hello world
0) lpushx key mylist value hello success!
1) lpushx key mylist value world success!
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
127.0.0.1:6379~[db0]#> lpush mylist hi
0) lpush key mylist value hi success!
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | hi   |
+-----+-------+
127.0.0.1:6379~[db0]#> rpushx mylist hello world
0) lpushx key mylist value hello success!
1) lpushx key mylist value world success!
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | world |
+-----+-------+
| 1   | hello |
+-----+-------+
| 2   | hi   |
+-----+-------+ 
```
	
	
## 	Rpush 命令用于将一个或多个值插入到列表的尾部(最右边)。
```bash
如果列表不存在，一个空列表会被创建并执行 RPUSH 操作。 当列表存在但不是列表类型时，返回一个错误。

注意：在 Redis 2.4 版本以前的 RPUSH 命令，都只接受单个 value 值。

语法
redis Rpush 命令基本语法如下：

redis 127.0.0.1:6379> RPUSH KEY_NAME VALUE1..VALUEN
可用版本
>= 1.0.0

返回值
执行 RPUSH 操作后，列表的长度。

实例

实例
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | bar   |
+-----+-------+
127.0.0.1:6379~[db0]#> rpush mylist hello 
0) rpush key mylist value hello success!
127.0.0.1:6379~[db0]#> lrange mylist 0 -1 
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | hello |
+-----+-------+
| 2   | bar   |
+-----+-------+
```


##  Rpoplpush 命令用于移除列表的最后一个元素，并将该元素添加到另一个列表并返回。
```bash
语法
redis Rpoplpush 命令基本语法如下：

redis 127.0.0.1:6379> RPOPLPUSH SOURCE_KEY_NAME DESTINATION_KEY_NAME
可用版本
>= 1.0.0

返回值
被弹出的元素。

实例
127.0.0.1:6379~[db0]#> lrange mylist 0 -1 
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | hello |
+-----+-------+
| 2   | bar   |
+-----+-------+
127.0.0.1:6379~[db0]#> rpoplpush mylist myotherlist 
RPOPLPUSH success value bar
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | hello |
+-----+-------+
127.0.0.1:6379~[db0]#> lrange myotherlist 0 -1 
+-----+-------+
| num | value |
+-----+-------+
| 0   | bar   |
+-----+-------+
```


##  Rpop 命令用于移除列表的最后一个元素，返回值为移除的元素。
```bash
语法
redis Rpop 命令基本语法如下：

redis 127.0.0.1:6379> RPOP KEY_NAME 
可用版本
>= 1.0.0

返回值
被移除的元素。

当列表不存在时，返回 nil 。

实例
127.0.0.1:6379~[db0]#> lrange mylist 0 -1 
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | bar   |
+-----+-------+
| 2   | world |
+-----+-------+
127.0.0.1:6379~[db0]#> rpop mylist
+----------+-------+
| list_key | value |
+----------+-------+
| mylist   | world |
+----------+-------+
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | bar   |
+-----+-------+
```




##  Ltrim 对一个列表进行修剪(trim)，就是说，让列表只保留指定区间内的元素，不在指定区间之内的元素都将被删除。
```bash
下标 0 表示列表的第一个元素，以 1 表示列表的第二个元素，以此类推。 你也可以使用负数下标，以 -1 表示列表的最后一个元素， -2 表示列表的倒数第二个元素，以此类推。

语法
redis Ltrim 命令基本语法如下：

redis 127.0.0.1:6379> LTRIM KEY_NAME START STOP
可用版本
>= 1.0.0

返回值
命令执行成功时，返回 ok 。

实例
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | bad   |
+-----+-------+
| 1   | bar   |
+-----+-------+
| 2   | world |
+-----+-------+
127.0.0.1:6379~[db0]#> ltrim mylist 1 -1 
LTRIM success
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | bar   |
+-----+-------+
| 1   | world |
+-----+-------+
```



## Lset 通过索引来设置元素的值。
```bash
当索引参数超出范围，或对一个空列表进行 LSET 时，返回一个错误。

关于列表下标的更多信息，请参考 LINDEX 命令。

语法
redis Lset 命令基本语法如下：

redis 127.0.0.1:6379> LSET KEY_NAME INDEX VALUE
可用版本
>= 1.0.0

返回值
操作成功返回 ok ，否则返回错误信息。

实例
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | bar   |
+-----+-------+
| 2   | world |
+-----+-------+
127.0.0.1:6379~[db0]#> lset mylist 0 bad   
LSET success
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | bad   |
+-----+-------+
| 1   | bar   |
+-----+-------+
| 2   | world |
+-----+-------+
```



##   Lrem 根据参数 COUNT 的值，移除列表中与参数 VALUE 相等的元素。
```bash
COUNT 的值可以是以下几种：

count > 0 : 从表头开始向表尾搜索，移除与 VALUE 相等的元素，数量为 COUNT 。
count < 0 : 从表尾开始向表头搜索，移除与 VALUE 相等的元素，数量为 COUNT 的绝对值。
count = 0 : 移除表中所有与 VALUE 相等的值。
语法
redis Lrem 命令基本语法如下：

redis 127.0.0.1:6379> LREM key count VALUE
可用版本
>= 1.0.0

返回值
被移除元素的数量。 列表不存在时返回 0 。

实例
127.0.0.1:6379~[db0]#> rpush mylist hello
 0) rpush key mylist value hello success!
127.0.0.1:6379~[db0]#> rpush mylist hello
 0) rpush key mylist value hello success!
127.0.0.1:6379~[db0]#> lrange mylist 0 -1 
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | hello |
+-----+-------+
| 2   | bar   |
+-----+-------+
| 3   | world |
+-----+-------+
| 4   | hello |
+-----+-------+
127.0.0.1:6379~[db0]#> lrem mylist -2 hello   
+--------+------------+
| key    | lrem_count |
+--------+------------+
| mylist | 2          |
+--------+------------+
127.0.0.1:6379~[db0]#> lrange mylist 0 -1   
+-----+-------+
| num | value |
+-----+-------+
| 0   | hello |
+-----+-------+
| 1   | bar   |
+-----+-------+
| 2   | world |
+-----+-------+
```




##  Brpoplpush 命令从列表中取出最后一个元素，并插入到另外一个列表的头部； 
如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止。
```bash
语法
redis Brpoplpush 命令基本语法如下：

redis 127.0.0.1:6379> BRPOPLPUSH LIST1 ANOTHER_LIST TIMEOUT 
可用版本
>= 2.0.0

返回值
假如在指定时间内没有任何元素被弹出，则返回一个 nil 和等待时长。 反之，返回一个
含有两个元素的列表，第一个元素是被弹出元素的值，第二个元素是等待时长。

实例
127.0.0.1:6379~[db0]#> brpoplpush mylist mylist_bak 10
+----------+-------+
| list_key | value |       
+----------+-------+       
| mylist   | test1 |       
+----------+-------+       
127.0.0.1:6379~[db0]#> exists mylist_bak
true
127.0.0.1:6379~[db0]#> lrange mylist_bak 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | test1 |
+-----+-------+
```



##  Lpushx 将一个值插入到已存在的列表头部，列表不存在时操作无效。
```bash
语法
redis Lpushx 命令基本语法如下：

redis 127.0.0.1:6379> LPUSHX KEY_NAME VALUE1.. VALUEN
可用版本
>= 2.2.0

返回值
LPUSHX 命令执行之后，列表的长度。

实例
127.0.0.1:6379~[db0]#> lpushx mylist bar
0) lpushx key mylist value bar success!
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | bar   |
+-----+-------+
| 1   | world |
+-----+-------+
| 2   | hello |
+-----+-------+
```


## Lpop 命令用于移除并返回列表的第一个元素。
```bash
语法
redis Lpop 命令基本语法如下：

redis 127.0.0.1:6379> Lpop KEY_NAME 
可用版本
>= 1.0.0

返回值
列表的第一个元素。 当列表 key 不存在时，返回 nil 。

实例
127.0.0.1:6379~[db0]#> lpop mylist
+----------+-------+
| list_key | value |
+----------+-------+
| mylist   | There |
+----------+-------+
```




##  Llen 命令用于返回列表的长度。 如果列表 key 不存在，则 key 被解释为一个空列表，返回 0 。 如果 key 不是列表类型，返回一个错误。
```bash
语法
redis Llen 命令基本语法如下：

redis 127.0.0.1:6379> LLEN KEY_NAME 
可用版本
>= 1.0.0

返回值
列表的长度。

实例
127.0.0.1:6379~[db0]#> llen mylist
+--------+--------+
| key    | length |
+--------+--------+
| mylist | 8      |
+--------+--------+ 
```
 

##   Linsert 命令用于在列表的元素前或者后插入元素。当指定元素不存在于列表中时，不执行任何操作。
```bash
当列表不存在时，被视为空列表，不执行任何操作。

如果 key 不是列表类型，返回一个错误。

语法
redis Linsert 命令基本语法如下：

LINSERT key BEFORE|AFTER pivot value
将值 value 插入到列表 key 当中，位于值 pivot 之前或之后。

可用版本
>= 1.0.0

返回值
如果命令执行成功，返回插入操作完成之后，列表的长度。 如果没有找到指定元素 ，返回 -1 。 如果 key 不存在或为空列表，返回 0 。

实例
127.0.0.1:6379~[db0]#> linsert mylist before world There 
LINSERT success
+--------+--------+
| key    | length |
+--------+--------+
| mylist | 8      |
+--------+--------+
127.0.0.1:6379~[db0]#> lrange mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | There |
+-----+-------+
| 1   | world |
+-----+-------+
| 2   | hello |
+-----+-------+ 
```


## Lindex 命令用于通过索引获取列表中的元素。你也可以使用负数下标，以 -1 表示列表的最后一个元素， -2 表示列表的倒数第二个元素，以此类推。
```bash
语法
redis Lindex 命令基本语法如下：

redis 127.0.0.1:6379> LINDEX KEY_NAME INDEX_POSITION 
可用版本
>= 1.0.0

返回值
列表中下标为指定索引值的元素。 如果指定索引值不在列表的区间范围内，返回 nil 。

实例
127.0.0.1:6379~[db0]#> lindex mylist -1
+--------+-------+-------+
| key    | index | value |
+--------+-------+-------+
| mylist | -1    | test1 |
+--------+-------+-------+
```


##  Brpop 命令移出并获取列表的最后一个元素， 如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止。
```bash
语法
redis Brpop 命令基本语法如下：

redis 127.0.0.1:6379> BRPOP LIST1 LIST2 .. LISTN TIMEOUT 
可用版本
>= 2.0.0

返回值
假如在指定时间内没有任何元素被弹出，则返回一个 nil 和等待时长。 反之，返回一个含有两个元素的列表，第一个元素是被弹出元素所属的 key ，第二个元素是被弹出元素的值。

实例
127.0.0.1:6379~[db0]#> brpop mylist 10
+----------+-------+
| list_key | value |
+----------+-------+
| mylist   | test6 |
+----------+-------+
```

## Blpop 命令移出并获取列表的第一个元素， 如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止。

语法
redis Blpop 命令基本语法如下：

redis 127.0.0.1:6379> BLPOP LIST1 LIST2 .. LISTN TIMEOUT
可用版本
>= 2.0.0

返回值
如果列表为空，返回一个 nil 。 否则，返回一个含有两个元素的列表，第一个元素是被弹出元素所属的 key ，第二个元素是被弹出元素的值。

实例
127.0.0.1:6379~[db0]#> blpop mylist 10
+----------+-------+
| list_key | value |
+----------+-------+
| mylist   | test6 |
+----------+-------+
```



##  Lrange 返回列表中指定区间内的元素，区间以偏移量 START 和 END 指定。 其中 0 表示列表的第一个元素
 1 表示列表的第二个元素，以此类推。 你也可以使用负数下标，以 -1 表示列表的最后一个元素， -2 表示列表的倒数第二个元素，以此类推。
```bash
语法
redis Lrange 命令基本语法如下：

redis 127.0.0.1:6379> LRANGE KEY_NAME START END
可用版本
>= 1.0.0

返回值
一个列表，包含指定区间内的元素。

实例
127.0.0.1:6379~[db0]#> lpush mylist test1 test2 test3 tes4 test5 test6
0) lpush key mylist value test1 success!
1) lpush key mylist value test2 success!
2) lpush key mylist value test3 success!
3) lpush key mylist value tes4 success! 
4) lpush key mylist value test5 success!
5) lpush key mylist value test6 success!
127.0.0.1:6379~[db0]#> LRANGE mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | test6 |
+-----+-------+
| 1   | test5 |
+-----+-------+
| 2   | tes4  |
+-----+-------+
| 3   | test3 |
+-----+-------+
| 4   | test2 |
+-----+-------+
| 5   | test1 |
+-----+-------+
```


## Lpush 命令将一个或多个值插入到列表头部。 如果 key 不存在，一个空列表会被创建并执行 LPUSH 操作。 当 key 存在但不是列表类型时，返回一个错误。
```bash
注意：在Redis 2.4版本以前的 LPUSH 命令，都只接受单个 value 值。

语法
redis Lpush 命令基本语法如下：

redis 127.0.0.1:6379> LPUSH KEY_NAME VALUE1.. VALUEN
可用版本
>= 1.0.0

返回值
执行 LPUSH 命令后，列表的长度。

实例
127.0.0.1:6379~[db0]#> lpush mylist test1 test2 test3 tes4 test5 test6
0) lpush key mylist value test1 success!
1) lpush key mylist value test2 success!
2) lpush key mylist value test3 success!
3) lpush key mylist value tes4 success! 
4) lpush key mylist value test5 success!
5) lpush key mylist value test6 success!
127.0.0.1:6379~[db0]#> LRANGE mylist 0 -1
+-----+-------+
| num | value |
+-----+-------+
| 0   | test6 |
+-----+-------+
| 1   | test5 |
+-----+-------+
| 2   | tes4  |
+-----+-------+
| 3   | test3 |
+-----+-------+
| 4   | test2 |
+-----+-------+
| 5   | test1 |
+-----+-------+
```

# Set
##  Sadd 命令将一个或多个成员元素加入到集合中，已经存在于集合的成员元素将被忽略
```bash
假如集合 key 不存在，则创建一个只包含添加的元素作成员的集合。

当集合 key 不是集合类型时，返回一个错误。

注意：在 Redis2.4 版本以前， SADD 只接受单个成员值。

语法
redis Sadd 命令基本语法如下：

redis 127.0.0.1:6379> SADD KEY_NAME VALUE1..VALUEN
可用版本
>= 1.0.0

返回值
被添加到集合中的新元素的数量，不包括被忽略的元素。

实例
127.0.0.1:6379~[db0]#> type 
+-------+------+
| key   | type |
+-------+------+
|  | set  |
+-------+------+
127.0.0.1:6379~[db0]#> smembers 
+----------+
| value    |
+----------+
| sogou  |
+----------+
| bing   |
+----------+
| Google |
+----------+
| baidu  |
+----------+
127.0.0.1:6379~[db0]#> sadd  soso
sadd success (1)
127.0.0.1:6379~[db0]#> smembers   
+----------+
| value    |
+----------+
| sogou  |
+----------+
| bing   |
+----------+
| Google |
+----------+
| baidu  |
+----------+
| soso     |
+----------+
```


##  Scard 命令返回集合中元素的数量。
```bash
语法
redis Scard 命令基本语法如下：

redis 127.0.0.1:6379> SCARD KEY_NAME 
可用版本
>= 1.0.0

返回值
集合的数量。 当集合 key 不存在时，返回 0 。

实例
127.0.0.1:6379~[db0]#> scard myset
+-------+------+-------+
| key   | type | value |
+-------+------+-------+
| myset | set  | 5     |
+-------+------+-------+
```

##  Sdiff 命令返回第一个集合与其他集合之间的差异，也可以认为说第一个集合中独有的元素。不存在的集合 key 将视为空集。
```bash
差集的结果来自前面的 FIRST_KEY ,而不是后面的 OTHER_KEY1，也不是整个 FIRST_KEY OTHER_KEY1..OTHER_KEYN 的差集。

实例:

key1 = {a,b,c,d]
key2 = {c]
key3 = {a,c,e]
SDIFF key1 key2 key3 = {b,d]
语法
redis Sdiff 命令基本语法如下：

redis 127.0.0.1:6379> SDIFF FIRST_KEY OTHER_KEY1..OTHER_KEYN 
可用版本
>= 1.0.0

返回值
包含差集成员的列表。

实例
127.0.0.1:6379~[db0]#> sadd k1 a   
sadd success (1)
127.0.0.1:6379~[db0]#> sadd k1 b
sadd success (1)
127.0.0.1:6379~[db0]#> sadd k1 c
sadd success (1)
127.0.0.1:6379~[db0]#> sadd k2 c 
sadd success (1)
127.0.0.1:6379~[db0]#> sadd k2 d
sadd success (1)
127.0.0.1:6379~[db0]#> sdiff k1 k2
+------------+
| diff-value |
+------------+
| a          |
+------------+
| b          |
+------------+
```


##  Sdiffstore 命令将给定集合之间的差集存储在指定的集合中。如果指定的集合 key 已存在，则会被覆盖。
```bash
语法
redis Sdiffstore 命令基本语法如下：

redis 127.0.0.1:6379> SDIFFSTORE DESTINATION_KEY KEY1..KEYN 
可用版本
>= 1.0.0

返回值
结果集中的元素数量。

实例
127.0.0.1:6379~[db0]#> sdiffstore k3 k1 k2
sdiffstore success key k3 (2)
127.0.0.1:6379~[db0]#> smembers k2
+-------+
| value |
+-------+
| d     |
+-------+
| c     |
+-------+
```


##  Sinter 命令返回给定所有给定集合的交集。 不存在的集合 key 被视为空集。 当给定集合当中有一个空集时，结果也为空集(根据集合运算定律)。

```bash
语法
redis Sinter 命令基本语法如下：

redis 127.0.0.1:6379> SINTER KEY KEY1..KEYN 
可用版本
>= 1.0.0

返回值
交集成员的列表。

实例
127.0.0.1:6379~[db0]#> sinter k1 k2    
+--------------+
| sinter-value |
+--------------+
| c            |
+--------------+
```


##  Sinterstore 命令将给定集合之间的交集存储在指定的集合中。如果指定的集合已经存在，则将其覆盖。
```bash
语法
redis Sinterstore 命令基本语法如下：

redis 127.0.0.1:6379> SINTERSTORE DESTINATION_KEY KEY KEY1..KEYN 
可用版本
>= 1.0.0

返回值
返回存储交集的集合的元素数量。

实例
127.0.0.1:6379~[db0]#> sinterstore k4  k1 k2 
sinterstore success key k4 (1)
127.0.0.1:6379~[db0]#> smembers k4
+-------+
| value |
+-------+
| c     |
+-------+
```


##  Sscan 命令用于迭代集合中键的元素，Sscan 继承自 Scan。
```bash
语法
redis Sscan 命令基本语法如下：

SSCAN key cursor [MATCH pattern] [COUNT count]
cursor - 游标。
pattern - 匹配的模式。
count - 指定从数据集里返回多少元素，默认值为 10 。
可用版本
>= 2.8.0

返回值
数组列表。

实例
127.0.0.1:6379~[db0]#> sscan k1 0 match *
+--------+--------------+
| number | union-member |
+--------+--------------+
| 0      | a            |
+--------+--------------+
| 1      | c            |
+--------+--------------+
| 2      | b            |
+--------+--------------+
```




##  Sunionstore 命令将给定集合的并集存储在指定的集合 destination 中。如果 destination 已经存在，则将其覆盖。
```bash
语法
redis Sunionstore 命令基本语法如下：

SUNIONSTORE destination key [key ...]
可用版本
>= 1.0.0

返回值
结果集中的元素数量。

实例
127.0.0.1:6379~[db0]#> sunionstore k5 k1 k2
sunionstore success integer (4)
127.0.0.1:6379~[db0]#> smembers k5
+-------+
| value |
+-------+
| a     |
+-------+
| d     |
+-------+
| b     |
+-------+
| c     |
+-------+
```



## Sunion 命令返回给定集合的并集。不存在的集合 key 被视为空集。
```bash
语法
redis Sunion 命令基本语法如下：

redis 127.0.0.1:6379> SUNION KEY KEY1..KEYN
可用版本
>= 1.0.0

返回值
并集成员的列表。

实例
127.0.0.1:6379~[db0]#> sunion  k1 k2      
+--------------+
| union-member |
+--------------+
| d            |
+--------------+
| c            |
+--------------+
```


## Srem 命令用于移除集合中的一个或多个成员元素，不存在的成员元素会被忽略。
```bash
当 key 不是集合类型，返回一个错误。

在 Redis 2.4 版本以前， SREM 只接受单个成员值。

语法
redis Srem 命令基本语法如下：

redis 127.0.0.1:6379> SREM KEY MEMBER1..MEMBERN
可用版本
>= 1.0.0

返回值
被成功移除的元素的数量，不包括被忽略的元素。

实例
127.0.0.1:6379~[db0]#> smembers k5
+-------+
| value |
+-------+
| a     |
+-------+
| d     |
+-------+
| b     |
+-------+
| c     |
+-------+
127.0.0.1:6379~[db0]#> srem k5 a
srem success integer (1)
127.0.0.1:6379~[db0]#> smembers k5
+-------+
| value |
+-------+
| d     |
+-------+
| b     |
+-------+
| c     |
+-------+
```


## Srandmember 命令用于返回集合中的一个随机元素。
```bash
从 Redis 2.6 版本开始， Srandmember 命令接受可选的 count 参数：

如果 count 为正数，且小于集合基数，那么命令返回一个包含 count 个元素的数组，数组中的元素各不相同。如果 count 大于等于集合基数，那么返回整个集合。
如果 count 为负数，那么命令返回一个数组，数组中的元素可能会重复出现多次，而数组的长度为 count 的绝对值。
该操作和 SPOP 相似，但 SPOP 将随机元素从集合中移除并返回，而 Srandmember 则仅仅返回随机元素，而不对集合进行任何改动。

语法
redis Srandmember 命令基本语法如下：

redis 127.0.0.1:6379> SRANDMEMBER KEY [count]
可用版本
>= 1.0.0

返回值
只提供集合 key 参数时，返回一个元素；如果集合为空，返回 nil 。 如果提供了 count 参数，那么返回一个数组；如果集合为空，返回空数组。

实例
10.161.55.194:6379~[db0]#> smembers k5
+-------+
| value |
+-------+
| d     |
+-------+
| b     |
+-------+
| c     |
+-------+
127.0.0.1:6379~[db0]#> srandmember k5
+-------------+
| rand-member |
+-------------+
| b           |
+-------------+
127.0.0.1:6379~[db0]#> srandmember k5
+-------------+
| rand-member |
+-------------+
| c           |
+-------------+
127.0.0.1:6379~[db0]#> srandmember k5
+-------------+
| rand-member |
+-------------+
| c           |
+-------------+
```


##  Spop 命令用于移除集合中的指定 key 的一个或多个随机元素，移除后会返回移除的元素。
```bash
该命令类似 Srandmember 命令，但 SPOP 将随机元素从集合中移除并返回，而 Srandmember 则仅仅返回随机元素，而不对集合进行任何改动。

语法
redis Spop 命令基本语法如下：

SPOP key [count]
count 参数在 3.2+ 版本可用。

可用版本
>= 1.0.0

返回值
被移除的随机元素。 当集合不存在或是空集时，返回 nil 。

实例
127.0.0.1:6379~[db0]#> smembers k5    
+-------+
| value |
+-------+
| d     |
+-------+
| c     |
+-------+
| b     |
+-------+
127.0.0.1:6379~[db0]#> spop k5
SPOP  Cli length is failed,length is  3
127.0.0.1:6379~[db0]#> spop k5 1
+-----------+
| pop-value |
+-----------+
| d         |
+-----------+
127.0.0.1:6379~[db0]#> smembers k5
+-------+
| value |
+-------+
| c     |
+-------+
| b     |
+-------+
```



## Smove 命令将指定成员 member 元素从 source 集合移动到 destination 集合。
```bash
SMOVE 是原子性操作。

如果 source 集合不存在或不包含指定的 member 元素，则 SMOVE 命令不执行任何操作，仅返回 0 。否则， member 元素从 source 集合中被移除，并添加到 destination 集合中去。

当 destination 集合已经包含 member 元素时， SMOVE 命令只是简单地将 source 集合中的 member 元素删除。

当 source 或 destination 不是集合类型时，返回一个错误。

语法
redis Smove 命令基本语法如下：

redis 127.0.0.1:6379> SMOVE SOURCE DESTINATION MEMBER 
可用版本
>= 1.0.0

返回值
如果成员元素被成功移除，返回 1 。 如果成员元素不是 source 集合的成员，并且没有任何操作对 destination 集合执行，那么返回 0 。

实例
127.0.0.1:6379~[db0]#> smembers k1
+-------+
| value |
+-------+
| d     |
+-------+
| c     |
+-------+
127.0.0.1:6379~[db0]#> smove k1 k6 d
smove success integer (1)
127.0.0.1:6379~[db0]#> smembers k1   
+-------+
| value |
+-------+
| c     |
+-------+
```


## Smembers 命令返回集合中的所有的成员。 不存在的集合 key 被视为空集合。
```bash
语法
redis Smembers 命令基本语法如下：

redis 127.0.0.1:6379> SMEMBERS key
可用版本
>= 1.0.0

返回值
集合中的所有成员。

实例
127.0.0.1:6379~[db0]#> smembers k1
+-------+
| value |
+-------+
| c     |
+-------+
```


##  Sismember 命令判断成员元素是否是集合的成员。
```bash
语法
redis Sismember 命令基本语法如下：

redis 127.0.0.1:6379> SISMEMBER KEY VALUE 
可用版本
>= 1.0.0

返回值
如果成员元素是集合的成员，返回 1 。 如果成员元素不是集合的成员，或 key 不存在，返回 0 。

实例
127.0.0.1:6379~[db0]#> smembers k1
+-------+
| value |
+-------+
| c     |
+-------+
127.0.0.1:6379~[db0]#> sismember k1 c
exists!
127.0.0.1:6379~[db0]#> sismember k1 a
none!
```

# Sorted Set
## Zadd 命令用于将一个或多个成员元素及其分数值加入到有序集当中。
```bash
如果某个成员已经是有序集的成员，那么更新这个成员的分数值，并通过重新插入这个成员元素，来保证该成员在正确的位置上。

分数值可以是整数值或双精度浮点数。

如果有序集合 key 不存在，则创建一个空的有序集并执行 ZADD 操作。

当 key 存在但不是有序集类型时，返回一个错误。

注意： 在 Redis 2.4 版本以前， ZADD 每次只能添加一个元素。

语法
redis Zadd 命令基本语法如下：

redis 127.0.0.1:6379> ZADD KEY_NAME SCORE1 VALUE1.. SCOREN VALUEN
可用版本
>= 1.2.0

返回值
被成功添加的新成员的数量，不包括那些被更新的、已经存在的成员。

实例
127.0.0.1:6379~[db0]#> zadd myzset 1 one  
ZADD integer (1)
127.0.0.1:6379~[db0]#> zrange myzset 0 -1
+--------------+
| range-member |
+--------------+
| one          |
+--------------+
```



## Zcard 命令用于计算集合中元素的数量。
```bash
语法
redis Zcard 命令基本语法如下：

redis 127.0.0.1:6379> ZCARD KEY_NAME
可用版本
>= 1.2.0

返回值
当 key 存在且是有序集类型时，返回有序集的基数。 当 key 不存在时，返回 0 。

实例
127.0.0.1:6379~[db0]#> zcard myzset
ZCARD integer (1)
127.0.0.1:6379~[db0]#> zrange myzset 0 -1
+--------------+
| range-member |
+--------------+
| one          |
+--------------+
```


## Zcount 命令用于计算有序集合中指定分数区间的成员数量。
```bash
语法
redis Zcount 命令基本语法如下：

redis 127.0.0.1:6379> ZCOUNT key min max
可用版本
>= 2.0.0

返回值
分数值在 min 和 max 之间的成员的数量。

实例
127.0.0.1:6379~[db0]#> zcount myzset 0 1
ZCOUNT integer (1)
```



## Zincrby 命令对有序集合中指定成员的分数加上增量 increment
```bash
可以通过传递一个负数值 increment ，让分数减去相应的值，比如 ZINCRBY key -5 member ，就是让 member 的 score 值减去 5 。

当 key 不存在，或分数不是 key 的成员时， ZINCRBY key increment member 等同于 ZADD key increment member 。

当 key 不是有序集类型时，返回一个错误。

分数值可以是整数值或双精度浮点数。

语法
redis Zincrby 命令基本语法如下：

redis 127.0.0.1:6379> ZINCRBY key increment member
可用版本
>= 1.2.0

返回值
member 成员的新分数值，以字符串形式表示。

实例
127.0.0.1:6379~[db0]#> zincrby myzset 2 two
ZINCRBY (4)
```



## Zinterstore 命令计算给定的一个或多个有序集的交集，其中给定 key 的数量必须以 numkeys 参数指定，并将该交集(结果集)储存到 destination 。
```bash
默认情况下，结果集中某个成员的分数值是所有给定集下该成员分数值之和。

语法
redis Zinterstore 命令基本语法如下：

redis 127.0.0.1:6379> ZINTERSTORE destination numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX]
可用版本
>= 2.0.0

返回值
保存到目标结果集的的成员数量。

实例
127.0.0.1:6379~[db0]#> ZADD mid_test 70 'Li Lei' 
ZADD integer (1)
127.0.0.1:6379~[db0]#> ZADD mid_test 70 'Han Meimei' 
ZADD integer (1)
127.0.0.1:6379~[db0]#> ZADD mid_test 99.5 'Tom' 
ZADD integer (1)
127.0.0.1:6379~[db0]#> ZADD fin_test 88 'Li Lei' 
ZADD integer (1)
127.0.0.1:6379~[db0]#> ZADD fin_test 75 'Han Meimei' 
ZADD integer (1)
127.0.0.1:6379~[db0]#> ZADD fin_test 99.5 'Tom' 
ZADD integer (1)
127.0.0.1:6379~[db0]#> ZINTERSTORE sum_point 2 mid_test fin_test
ZINTERSTORE integer (3)
127.0.0.1:6379~[db0]#> zrange sum_point  0 -1 
+--------------+
| range-member |
+--------------+
| Han Meimei   |
+--------------+
| Li Lei       |
+--------------+
| Tom          |
+--------------+
```


## Zlexcount 命令在计算有序集合中指定字典区间内成员数量。
```bash
语法
redis Zlexcount 命令基本语法如下：

redis 127.0.0.1:6379> ZLEXCOUNT KEY MIN MAX
可用版本
>= 2.8.9

返回值
指定区间内的成员数量。

实例
127.0.0.1:6379~[db0]#> zrange sum_point  0 -1 
+--------------+
| range-member |
+--------------+
| Han Meimei   |
+--------------+
| Li Lei       |
+--------------+
| Tom          |
+--------------+
127.0.0.1:6379~[db0]#> zlexcount myzset - +
ZLEXCOUNT integer (3)
```


## Zrange 返回有序集中，指定区间内的成员。
```bash
其中成员的位置按分数值递增(从小到大)来排序。

具有相同分数值的成员按字典序(lexicographical order )来排列。

如果你需要成员按

值递减(从大到小)来排列，请使用 ZREVRANGE 命令。

下标参数 start 和 stop 都以 0 为底，也就是说，以 0 表示有序集第一个成员，以 1 表示有序集第二个成员，以此类推。

你也可以使用负数下标，以 -1 表示最后一个成员， -2 表示倒数第二个成员，以此类推。

语法
redis Zrange 命令基本语法如下：

redis 127.0.0.1:6379> ZRANGE key start stop [WITHSCORES]
可用版本
>= 1.2.0

返回值
指定区间内，带有分数值(可选)的有序集成员的列表。

实例
127.0.0.1:6379~[db0]#> zrange sum_point  0 -1 
+--------------+
| range-member |
+--------------+
| Han Meimei   |
+--------------+
| Li Lei       |
+--------------+
| Tom          |
+--------------+
```


## Zrangebylex 通过字典区间返回有序集合的成员。
```bash
语法
redis Zrange 命令基本语法如下：

redis 127.0.0.1:6379> ZRANGEBYLEX key min max [LIMIT offset count]
可用版本
>= 2.8.9

返回值
指定区间内的元素列表。

实例
127.0.0.1:6379~[db0]#> zadd myzset 0 a 0 b 0 c 0 d 0 e 0 f 0 g 
ZADD integer (7)
127.0.0.1:6379~[db0]#> zrangebylex myzset - [c 
+-------------------+
| rangebylex-member |
+-------------------+
| a                 |
+-------------------+
| b                 |
+-------------------+
| c                 |
+-------------------+
```



## Zrangebyscore 返回有序集合中指定分数区间的成员列表。有序集成员按分数值递增(从小到大)次序排列。
```bash
具有相同分数值的成员按字典序来排列(该属性是有序集提供的，不需要额外的计算)。

默认情况下，区间的取值使用闭区间 (小于等于或大于等于)，你也可以通过给参数前增加 ( 符号来使用可选的开区间 (小于或大于)。

举个例子：

ZRANGEBYSCORE zset (1 5
返回所有符合条件 1 < score <= 5 的成员，而

ZRANGEBYSCORE zset (5 (10
则返回所有符合条件 5 < score < 10 的成员。

语法
redis Zrangebyscore 命令基本语法如下：

redis 127.0.0.1:6379> ZRANGEBYSCORE key min max [WITHSCORES] [LIMIT offset count]
可用版本
>= 1.0.5

返回值
指定区间内，带有分数值(可选)的有序集成员的列表。

实例
127.0.0.1:6379~[db0]#> zrangebyscore  myzset -inf +inf 
+---------------------+
| rangebyscore-member |
+---------------------+
| a                   |
+---------------------+
| b                   |
+---------------------+
| c                   |
+---------------------+
| d                   |
+---------------------+
| e                   |
+---------------------+
| f                   |
+---------------------+
| g                   |
+---------------------+
```



## Zrank 返回有序集中指定成员的排名。其中有序集成员按分数值递增(从小到大)顺序排列。
```bash
语法
redis Zrank 命令基本语法如下：

redis 127.0.0.1:6379> ZRANK key member
可用版本
>= 2.0.0

返回值
如果成员是有序集 key 的成员，返回 member 的排名。 如果成员不是有序集 key 的成员，返回 nil 。

实例
127.0.0.1:6379~[db0]#> zrangebyscore  myzset -inf +inf 
+---------------------+
| rangebyscore-member |
+---------------------+
| a                   |
+---------------------+
| b                   |
+---------------------+
| c                   |
+---------------------+
| d                   |
+---------------------+
| e                   |
+---------------------+
| f                   |
+---------------------+
| g                   |
+---------------------+
127.0.0.1:6379~[db0]#> zrank myzset a 
ZRANK integer (0)
127.0.0.1:6379~[db0]#> zrank myzset d
ZRANK integer (3)
```


## Zrem 命令用于移除有序集中的一个或多个成员，不存在的成员将被忽略。
```bash
当 key 存在但不是有序集类型时，返回一个错误。

注意： 在 Redis 2.4 版本以前， ZREM 每次只能删除一个元素。

语法
redis Zrem 命令基本语法如下：

redis 127.0.0.1:6379> ZREM key member [member ...]
可用版本
>= 1.2.0

返回值
被成功移除的成员的数量，不包括被忽略的成员。

实例
127.0.0.1:6379~[db0]#> zrange  myzset 0 -1        
+--------------+
| range-member |
+--------------+
| a            |
+--------------+
| b            |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
| f            |
+--------------+
| g            |
+--------------+
127.0.0.1:6379~[db0]#> zrem myzset g
ZREM integer (1)
127.0.0.1:6379~[db0]#> zrange  myzset 0 -1
+--------------+
| range-member |
+--------------+
| a            |
+--------------+
| b            |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
| f            |
+--------------+
```


## Zremrangebylex 命令用于移除有序集合中给定的字典区间的所有成员。
```bash
语法
redis Zremrangebylex命令基本语法如下：

redis 127.0.0.1:6379> ZREMRANGEBYLEX key min max
可用版本
>= 2.8.9

返回值
被成功移除的成员的数量，不包括被忽略的成员。

实例
127.0.0.1:6379~[db0]#> zrange  myzset 0 -1
+--------------+
| range-member |
+--------------+
| a            |
+--------------+
| b            |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
| f            |
+--------------+
127.0.0.1:6379~[db0]#> zremrangebylex  myzset [a [d
ZREMRANGEBYLEX integer (4)
127.0.0.1:6379~[db0]#> zrange  myzset 0 -1          
+--------------+
| range-member |
+--------------+
| e            |
+--------------+
| f            |
+--------------+
```




## Zremrangebyrank 命令用于移除有序集中，指定排名(rank)区间内的所有成员。
```bash
语法
redis Zremrangebyrank 命令基本语法如下：

redis 127.0.0.1:6379> ZREMRANGEBYRANK key start stop
可用版本
>= 2.0.0

返回值
被移除成员的数量。

实例
127.0.0.1:6379~[db0]#> zrange myzset 0 -1            
+--------------+
| range-member |
+--------------+
| a            |
+--------------+
| b            |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
| f            |
+--------------+
| g            |
+--------------+
127.0.0.1:6379~[db0]#> zremrangebyrank myzset 0 1
ZREMRANGEBYRANK integer (2)
127.0.0.1:6379~[db0]#> zrange myzset 0 -1
+--------------+
| range-member |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
| f            |
+--------------+
| g            |
+--------------+
```



## Zremrangebyscore 命令用于移除有序集中，指定分数（score）区间内的所有成员。
```bash
语法
redis Zremrangebyscore 命令基本语法如下：

redis 127.0.0.1:6379> ZREMRANGEBYSCORE key min max
可用版本
>= 1.2.0

返回值
被移除成员的数量。

实例
127.0.0.1:6379~[db0]#> zrange myzset 0 -1
+--------------+
| range-member |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
| f            |
+--------------+
| g            |
+--------------+
127.0.0.1:6379~[db0]#> zremrangebyscore myzset 0 1
ZREMRANGEBYSCORE integer (5)
127.0.0.1:6379~[db0]#> zrange myzset 0 -1
+--------------+
| range-member |
+--------------+
```



## Zrevrange 命令返回有序集中，指定区间内的成员。
```bash
其中成员的位置按分数值递减(从大到小)来排列。

具有相同分数值的成员按字典序的逆序(reverse lexicographical order)排列。

除了成员按分数值递减的次序排列这一点外， ZREVRANGE 命令的其他方面和 ZRANGE 命令一样。

语法
redis Zrevrange 命令基本语法如下：

redis 127.0.0.1:6379> ZREVRANGE key start stop [WITHSCORES]
可用版本
>= 1.2.0

返回值
指定区间内，带有分数值(可选)的有序集成员的列表。

实例
127.0.0.1:6379~[db0]#> zadd myzset 0 a 0 b 0 c 0 d 0 e 0 f 0 g
ZADD integer (7)
127.0.0.1:6379~[db0]#> zrevrange  myzset  0 -1
+------------------+
| zrevrange-member |
+------------------+
| g                |
+------------------+
| f                |
+------------------+
| e                |
+------------------+
| d                |
+------------------+
| c                |
+------------------+
| b                |
+------------------+
| a                |
+------------------+
```



## Zscore 命令返回有序集中，成员的分数值。 如果成员元素不是有序集 key 的成员，或 key 不存在，返回 nil 。
```bash
语法
redis Zscore 命令基本语法如下：

redis 127.0.0.1:6379> ZSCORE key member
可用版本
>= 1.2.0

返回值
成员的分数值，以字符串形式表示。

实例
127.0.0.1:6379~[db0]#> zrange myzset 0 -1
+--------------+
| range-member |
+--------------+
| a            |
+--------------+
| f            |
+--------------+
| g            |
+--------------+
| b            |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
127.0.0.1:6379~[db0]#> ZSCORE myzset c   
ZSCORE (2)
```


## Zunionstore 命令计算给定的一个或多个有序集的并集，其中给定 key 的数量必须以 numkeys 参数指定，并将该并集(结果集)储存到 destination 。
```bash
默认情况下，结果集中某个成员的分数值是所有给定集下该成员分数值之和 。

语法
redis Zunionstore 命令基本语法如下：

redis 127.0.0.1:6379> ZUNIONSTORE destination numkeys key [key ...] [WEIGHTS weight [weight ...]] [AGGREGATE SUM|MIN|MAX]
可用版本
>= 2.0.0

返回值
保存到 destination 的结果集的成员数量。

实例
127.0.0.1:6379~[db0]#> zrange myzset 0 -1                            
+--------------+
| range-member |
+--------------+
| a            |
+--------------+
| f            |
+--------------+
| g            |
+--------------+
| b            |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
127.0.0.1:6379~[db0]#> zrange myzset_b 0 -1 
+--------------+
| range-member |
+--------------+
| four         |
+--------------+
| five         |
+--------------+
127.0.0.1:6379~[db0]#> zunionstore outzset 2 myzset myzset_b WEIGHTS 2 3 
ZUNIONSTORE integer (9)
127.0.0.1:6379~[db0]#> zrange outzset 0 -1  
+--------------+
| range-member |
+--------------+
| a            |
+--------------+
| f            |
+--------------+
| g            |
+--------------+
| b            |
+--------------+
| four         |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| five         |
+--------------+
| e            |
+--------------+
```




## Zscan 命令用于迭代有序集合中的元素（包括元素成员和元素分值）
```bash
语法
redis Zscan 命令基本语法如下：

redis 127.0.0.1:6379> ZSCAN key cursor [MATCH pattern] [COUNT count]
cursor - 游标。
pattern - 匹配的模式。
count - 指定从数据集里返回多少元素，默认值为 10 。
可用版本
>= 2.8.0

返回值
返回的每个元素都是一个有序集合元素，一个有序集合元素由一个成员（member）和一个分值（score）组成。

实例
127.0.0.1:6379~[db0]#> ZSCAN outzset 0 match *
+--------+-------------+
| number | zscan-value |
+--------+-------------+
| 0      | a           |
+--------+-------------+
| 1      | 0           |
+--------+-------------+
| 2      | f           |
+--------+-------------+
| 3      | 0           |
+--------+-------------+
| 4      | g           |
+--------+-------------+
| 5      | 0           |
+--------+-------------+
| 6      | b           |
+--------+-------------+
| 7      | 2           |
+--------+-------------+
| 8      | four        |
+--------+-------------+
| 9      | 3           |
+--------+-------------+
| 10     | c           |
+--------+-------------+
| 11     | 4           |
+--------+-------------+
| 12     | d           |
+--------+-------------+
| 13     | 6           |
+--------+-------------+
| 14     | five        |
+--------+-------------+
| 15     | 6           |
+--------+-------------+
| 16     | e           |
+--------+-------------+
| 17     | 8           |
+--------+-------------+
```





## Zrevrank 命令返回有序集中成员的排名。其中有序集成员按分数值递减(从大到小)排序。
```bash
排名以 0 为底，也就是说， 分数值最大的成员排名为 0 。

使用 ZRANK 命令可以获得成员按分数值递增(从小到大)排列的排名。

语法
redis Zrevrank 命令基本语法如下：

redis 127.0.0.1:6379> ZREVRANK key member
可用版本
>= 2.2.0

返回值
如果成员是有序集 key 的成员，返回成员的排名。 如果成员不是有序集 key 的成员，返回 nil 。

实例
127.0.0.1:6379~[db0]#> zrange myzset 0 -1
+--------------+
| range-member |
+--------------+
| a            |
+--------------+
| f            |
+--------------+
| g            |
+--------------+
| b            |
+--------------+
| c            |
+--------------+
| d            |
+--------------+
| e            |
+--------------+
127.0.0.1:6379~[db0]#> zrevrank myzset g  
ZREVRANK integer (4)
```


## Zrevrangebyscore 返回有序集中指定分数区间内的所有的成员。有序集成员按分数值递减(从大到小)的次序排列。
```bash
具有相同分数值的成员按字典序的逆序(reverse lexicographical order )排列。

除了成员按分数值递减的次序排列这一点外， ZREVRANGEBYSCORE 命令的其他方面和 ZRANGEBYSCORE 命令一样。

语法
redis Zrevrangebyscore 命令基本语法如下：

redis 127.0.0.1:6379> ZREVRANGEBYSCORE key max min [WITHSCORES] [LIMIT offset count]
可用版本
>= 2.2.0

返回值
指定区间内，带有分数值(可选)的有序集成员的列表。

实例
127.0.0.1:6379~[db0]#> zrevrangebyscore myzset 0  -1
+-------------------------+
| zrevrangebyscore-member |
+-------------------------+
| g                       |
+-------------------------+
| f                       |
+-------------------------+
| a                       |
+-------------------------+
```