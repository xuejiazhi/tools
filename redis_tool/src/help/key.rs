#[derive(Debug, Clone)]
pub struct KeyHelp {
}

pub trait Help {
    fn help_del(&self);  
    fn help_dump(&self);
    fn help_exists(&self);
    fn help_expire(&self);
    fn help_expireat(&self);
    fn help_pexpire(&self);
    fn help_pexpireat(&self);
}

impl Help for KeyHelp {
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
    
    fn help_exists(&self){
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
redis 127.0.0.1:6379>	
            ")
    }

    fn help_expire(&self){
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
fn help_expireat(&self){
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
    
10.161.55.194:6379~[db0]#> SET mykey `Hello`
+------+-------+---------+---------+
| walk | key   | val     | result  |
+------+-------+---------+---------+
| SET  | mykey | `Hello` | Success |
+------+-------+---------+---------+
## 为 key 设置过期时间：
    
10.161.55.194:6379~[db0]#> expireat mykey 1669356402000
Expireat success
+-------+-------+------------+-----+
| key   | val   | ttl        | err |
+-------+-------+------------+-----+
| mykey | hello | 1249705899 | nil |
+-------+-------+------------+-----+
    ")
}

fn help_pexpire(&self){
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

fn help_pexpireat(&self){
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
