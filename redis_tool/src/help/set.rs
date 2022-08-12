#[derive(Debug, Clone)]
pub struct SetHelp {}

//todo:
pub trait Help {
    fn help_sadd(&self);
    fn help_scard(&self);
    fn help_sdiff(&self);
    fn help_sdiffstore(&self);
    fn help_sinter(&self);
    fn help_sinterstore(&self);
    fn help_sscan(&self);
    fn help_sunionstore(&self);
    fn help_sunion(&self);
    fn help_srem(&self);
    fn help_srandmember(&self);
    fn help_spop(&self);
    fn help_smove(&self);
    fn help_smembers(&self);
    fn help_sismember(&self);
}

impl Help for SetHelp {
    fn help_sadd(&self) {
        println!(
            "
Sadd 命令将一个或多个成员元素加入到集合中，已经存在于集合的成员元素将被忽略。

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
127.0.0.1:6379~[db0]#> type myset
+-------+------+
| key   | type |
+-------+------+
| myset | set  |
+-------+------+
127.0.0.1:6379~[db0]#> smembers myset
+----------+
| value    |
+----------+
| sogou    |
+----------+
| bing     |
+----------+
| Google   |
+----------+
| baidu    |
+----------+
127.0.0.1:6379~[db0]#> sadd myset soso
sadd success (1)
127.0.0.1:6379~[db0]#> smembers myset  
+----------+
| value    |
+----------+
| sogou    |
+----------+
| bing     |
+----------+
|  Google  |
+----------+
| baidu    |
+----------+
| soso     |
+----------+
"
        )
    }

    fn help_scard(&self) {
        println!(
            "
Redis Scard 命令返回集合中元素的数量。

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
"
        )
    }

    fn help_sdiff(&self) {
        println!("
Sdiff 命令返回第一个集合与其他集合之间的差异，也可以认为说第一个集合中独有的元素。不存在的集合 key 将视为空集。

差集的结果来自前面的 FIRST_KEY ,而不是后面的 OTHER_KEY1，也不是整个 FIRST_KEY OTHER_KEY1..OTHER_KEYN 的差集。

实例:

key1 = [a,b,c,d]
key2 = [c]
key3 = [a,c,e]
SDIFF key1 key2 key3 = [b,d]
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
")
    }

    fn help_sdiffstore(&self) {
        println!(
            "
Sdiffstore 命令将给定集合之间的差集存储在指定的集合中。如果指定的集合 key 已存在，则会被覆盖。

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
"
        )
    }

    fn help_sinter(&self) {
        println!("
Sinter 命令返回给定所有给定集合的交集。 不存在的集合 key 被视为空集。 当给定集合当中有一个空集时，结果也为空集(根据集合运算定律)。

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
")
    }

    fn help_sinterstore(&self) {
        println!(
            "
Sinterstore 命令将给定集合之间的交集存储在指定的集合中。如果指定的集合已经存在，则将其覆盖。

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
"
        )
    }

    fn help_sscan(&self) {
        println!(
            "
Sscan 命令用于迭代集合中键的元素，Sscan 继承自 Scan。

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
        "
        )
    }

    fn help_sunionstore(&self) {
        println!("
Sunionstore 命令将给定集合的并集存储在指定的集合 destination 中。如果 destination 已经存在，则将其覆盖。
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
")
    }

    fn help_sunion(&self) {
        println!(
            "
Sunion 命令返回给定集合的并集。不存在的集合 key 被视为空集。

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
"
        )
    }

    fn help_srem(&self) {
        println!(
            "
Srem 命令用于移除集合中的一个或多个成员元素，不存在的成员元素会被忽略。

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
"
        )
    }

    fn help_srandmember(&self) {
        println!(
"
Srandmember 命令用于返回集合中的一个随机元素。
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
")
    }

    fn help_spop(&self) {
        println!("
Spop 命令用于移除集合中的指定 key 的一个或多个随机元素，移除后会返回移除的元素。

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
")
    }

    fn help_smove(&self) {
        println!("
Smove 命令将指定成员 member 元素从 source 集合移动到 destination 集合。

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
")
    }
    fn help_smembers(&self) {
        println!(
            "
Smembers 命令返回集合中的所有的成员。 不存在的集合 key 被视为空集合。
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
"
        )
    }
    fn help_sismember(&self) {
        println!(
            "
Sismember 命令判断成员元素是否是集合的成员。

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
        "
        )
    }
}
