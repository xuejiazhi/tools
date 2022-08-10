# redis_tool 简价

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
**Keys** 

---
title: Type 命令用于返回 key 所储存的值的类型
---

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
---
title: DUMP 命令用于序列化给定 key ,并返回被序列化的值
---
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
---
title: EXISTS 命令用于检查给定 key 是否存在
---
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

---
title: Expire 命令用于设置 key 的过期时间，key 过期后将不再可用。单位以秒计
---
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
---
title: Expireat 命令用于以 UNIX 时间戳(unix timestamp)格式设置 key 的过期时间。key 过期后将不再可用
---
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

---
title: PEXPIRE 命令和 EXPIRE 命令的作用类似，但是它以毫秒为单位设置 key 的生存时间，而不像 EXPIRE 命令那样，以秒为单位
---
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

---
title: PEXPIREAT 命令用于设置 key 的过期时间，以毫秒计。key 过期后将不再可用
---
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

