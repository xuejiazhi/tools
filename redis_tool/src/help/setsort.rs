#[derive(Debug, Clone)]
pub struct SetSortHelp {}

pub trait Help {
    fn help_zscan(&self);
    fn help_zunionstore(&self);
    fn help_zscore(&self);
    fn help_zrevrange(&self);
    fn help_zremrangebyscore(&self);
    fn help_zremrangebyrank(&self);
    fn help_zremrangebylex(&self);
    fn help_zrem(&self);
    fn help_zrank(&self);
    fn help_zrangebyscore(&self);
    fn help_zrangebylex(&self);
    fn help_zrange(&self);
    fn help_zlexcount(&self);
    fn help_zinterstore(&self);
    fn help_zincrby(&self);
    fn help_zcount(&self);
    fn help_zcard(&self);
    fn help_zadd(&self);

    fn help_zrevrangebyscore(&self);
    fn help_zrevrank(&self);
}

impl Help for SetSortHelp {
    fn help_zrevrangebyscore(&self) {
        println!("
Zrevrangebyscore 返回有序集中指定分数区间内的所有的成员。有序集成员按分数值递减(从大到小)的次序排列。
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
+-------------------------+")
    }

    fn help_zrevrank(&self) {
        println!(
            "
Zrevrank 命令返回有序集中成员的排名。其中有序集成员按分数值递减(从大到小)排序。

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
        "
        )
    }

    fn help_zscan(&self) {
        println!(
            "
Zscan 命令用于迭代有序集合中的元素（包括元素成员和元素分值）

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
        "
        )
    }

    fn help_zunionstore(&self) {
        println!("
Zunionstore 命令计算给定的一个或多个有序集的并集，其中给定 key 的数量必须以 numkeys 参数指定，并将该并集(结果集)储存到 destination 。

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
        ")
    }

    fn help_zscore(&self) {
        println!("
Zscore 命令返回有序集中，成员的分数值。 如果成员元素不是有序集 key 的成员，或 key 不存在，返回 nil 。

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
127.0.0.1:6379~[db0]#> zscore myzset c   
 ZSCORE (2)
")
    }

    fn help_zrevrange(&self) {
        println!(
            "
Zrevrange 命令返回有序集中，指定区间内的成员。

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
        "
        )
    }

    fn help_zremrangebyscore(&self) {
        println!(
            "
Zremrangebyscore 命令用于移除有序集中，指定分数（score）区间内的所有成员。

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
        "
        )
    }

    fn help_zremrangebyrank(&self) {
        println!(
            "
Zremrangebyrank 命令用于移除有序集中，指定排名(rank)区间内的所有成员。

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
+--------------+"
        )
    }

    fn help_zremrangebylex(&self) {
        println!(
            "
Zremrangebylex 命令用于移除有序集合中给定的字典区间的所有成员。

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
        "
        )
    }

    fn help_zrem(&self) {
        println!(
            "
Zrem 命令用于移除有序集中的一个或多个成员，不存在的成员将被忽略。

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
        "
        )
    }

    fn help_zrank(&self) {
        println!(
            "
Zrank 返回有序集中指定成员的排名。其中有序集成员按分数值递增(从小到大)顺序排列。

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
        
        "
        )
    }

    fn help_zrangebyscore(&self) {
        println!("
Zrangebyscore 返回有序集合中指定分数区间的成员列表。有序集成员按分数值递增(从小到大)次序排列。

具有相同分数值的成员按字典序来排列(该属性是有序集提供的，不需要额外的计算)。

默认情况下，区间的取值使用闭区间 (小于等于或大于等于)，你也可以通过给参数前增加 ( 符号来使用可选的开区间 (小于或大于)。

举个例子：
abs
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
        ")
    }

    fn help_zrangebylex(&self) {
        println!(
            "
Zrangebylex 通过字典区间返回有序集合的成员。

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
        "
        )
    }

    fn help_zrange(&self) {
        println!("
Zrange 返回有序集中，指定区间内的成员。

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
        ")
    }

    fn help_zlexcount(&self) {
        println!(
            "
Zlexcount 命令在计算有序集合中指定字典区间内成员数量。

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
        "
        )
    }

    fn help_zinterstore(&self) {
        println!("
Zinterstore 命令计算给定的一个或多个有序集的交集，其中给定 key 的数量必须以 numkeys 参数指定，并将该交集(结果集)储存到 destination 。

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
        ")
    }

    fn help_zincrby(&self) {
        println!("
Zincrby 命令对有序集合中指定成员的分数加上增量 increment

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
        ")
    }

    fn help_zcount(&self) {
        println!(
            "
Zcount 命令用于计算有序集合中指定分数区间的成员数量。
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
        "
        )
    }

    fn help_zcard(&self) {
        println!(
            "
Zcard 命令用于计算集合中元素的数量。

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
        "
        )
    }

    fn help_zadd(&self) {
        println!("
Zadd 命令用于将一个或多个成员元素及其分数值加入到有序集当中。

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
10.161.55.194:6379~[db0]#> zadd myzset 1 one  
ZADD integer (1)
10.161.55.194:6379~[db0]#> zrange myzset 0 -1
+--------------+
| range-member |
+--------------+
| one          |
+--------------+
        ")
    }
}
