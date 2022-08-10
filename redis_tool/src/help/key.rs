#[derive(Debug, Clone)]
pub struct KeyHelp {}

pub trait Help {
    fn help_del(&self);
    fn help_dump(&self);
    fn help_exists(&self);
    fn help_expire(&self);
    fn help_expireat(&self);
    fn help_pexpire(&self);
    fn help_pexpireat(&self);
    fn help_keys(&self);
    fn help_move(&self);
    fn help_persist(&self);
    fn help_pttl(&self);
    fn help_ttl(&self);
    fn help_randomkey(&self);
    fn help_rename(&self);
    fn help_renamenx(&self);
    fn help_scan(&self);
    fn help_type(&self);
}

impl Help for KeyHelp {
    fn help_type(&self) {
        println!(
            "
    ### Redis Type 命令用于返回 key 所储存的值的类型。

    ### 语法
    ### redis Renamenx 命令基本语法如下：
    
    redis 127.0.0.1:6379> TYPE KEY_NAME 
    ### 可用版本
    >= 1.0.0
    
    ### 返回值
    ### 返回 key 的数据类型，数据类型有：
    
    none (key不存在)
    string (字符串)
    list (列表)
    set (集合)
    zset (有序集)
    hash (哈希表)
    
    ### 实例
    127.0.0.1 :6379~[db0]#> type my_key
    +--------+--------+
    | key    | type   |        
    +--------+--------+        
    | my_key | string |        
    +--------+--------+  
        "
        )
    }
    fn help_scan(&self) {
        println!("
### Redis Scan 命令用于迭代数据库中的数据库键。

### SCAN 命令是一个基于游标的迭代器，每次被调用之后， 都会向用户返回一个新的游标， 用户在下次迭代时需要使用这个新游标作为 SCAN 命令的游标参数， 以此来延续之前的迭代过程。

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
+--------+--------+")
    }

    fn help_renamenx(&self) {
        println!(
            "
### Redis Renamenx 命令用于在新的 key 不存在时修改 key 的名称 。

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
    "
        )
    }

    fn help_rename(&self) {
        println!("
### Redis Rename 命令用于修改 key 的名称 。

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
        ")
    }

    fn help_randomkey(&self) {
        println!(
            "
### Redis RANDOMKEY 命令从当前数据库中随机返回一个 key 。

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
"
        )
    }

    fn help_ttl(&self) {
        println!("
        ### Redis TTL 命令以秒为单位返回 key 的剩余过期时间。

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
-1 <seconds>")
    }

    fn help_pttl(&self) {
        println!("
    Redis pttl命令以毫秒为单位返回 key 的剩余过期时间。

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
    ")
    }

    fn help_persist(&self) {
        println!(
            "
## Redis PERSIST 命令用于移除给定 key 的过期时间，使得 key 永不过期。

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

        "
        )
    }

    fn help_move(&self) {
        println!(
            "
### Redis MOVE 命令用于将当前数据库的 key 移动到给定的数据库 db 当中。

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
        "
        )
    }

    fn help_keys(&self) {
        println!(
            "
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
"
        )
    }

    fn help_del(&self) {
        println!(
            "
## Redis DEL 命令用于删除已存在的键。不存在的 key 会被忽略。

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
        "
        )
    }

    fn help_dump(&self) {
        println!(
            "	
## Redis DUMP 命令用于序列化给定 key ,并返回被序列化的值。

## 语法
redis DUMP 命令基本语法如下：

    redis 127.0.0.1:6379> DUMP KEY_NAME
## 可用版本
    >= 2.6.0

## 返回值
    如果 key 不存在，那么返回 nil 。 否则，返回序列化之后的值。

## 实例
    首先，我们在 redis 中创建一个 key 并设置值。

    redis> SET greeting 'hello, dumping world!'
    OK
现在使用 DUMP 序列化键值。

redis> DUMP greeting
\\x00\\x15hello, dumping world!\\x06\\x00E\\xa0Z\\x82\\xd8r\\xc1\\xde

redis> DUMP not-exists-key
(nil)
"
        )
    }

    fn help_exists(&self) {
        println!(
            "
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
            "
        )
    }

    fn help_expire(&self) {
        println!("
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
        ")
    }
    fn help_expireat(&self) {
        println!("
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
    ")
    }

    fn help_pexpire(&self) {
        println!("
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
    ")
    }

    fn help_pexpireat(&self) {
        println!("
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
    ")
    }
}
