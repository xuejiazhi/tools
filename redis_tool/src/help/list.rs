#[derive(Debug, Clone)]
pub struct ListHelp {}

pub trait Help {
    fn help_lpush(&self);
    fn help_lrange(&self);
    fn help_blpop(&self);
    fn help_brpop(&self);
    fn help_lindex(&self);
    fn help_linsert(&self);
    fn help_llen(&self);
    fn help_lpop(&self);
}

impl Help for ListHelp {
    fn help_lpop(&self) {
        println!(
            "
Redis Lpop 命令用于移除并返回列表的第一个元素。

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
    "
        )
    }

    fn help_llen(&self) {
        println!("
Redis Llen 命令用于返回列表的长度。 如果列表 key 不存在，则 key 被解释为一个空列表，返回 0 。 如果 key 不是列表类型，返回一个错误。

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
    ")
    }

    fn help_linsert(&self) {
        println!("
Redis Linsert 命令用于在列表的元素前或者后插入元素。当指定元素不存在于列表中时，不执行任何操作。

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
    ")
    }

    fn help_lindex(&self) {
        println!("
Redis Lindex 命令用于通过索引获取列表中的元素。你也可以使用负数下标，以 -1 表示列表的最后一个元素， -2 表示列表的倒数第二个元素，以此类推。

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
    ")
    }

    fn help_brpop(&self) {
        println!("
Redis Brpop 命令移出并获取列表的最后一个元素， 如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止。

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
")
    }

    fn help_blpop(&self) {
        println!("
Redis Blpop 命令移出并获取列表的第一个元素， 如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止。

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
    ")
    }

    fn help_lrange(&self) {
        println!("
Redis Lrange 返回列表中指定区间内的元素，区间以偏移量 START 和 END 指定。 其中 0 表示列表的第一个元素， 1 表示列表的第二个元素，以此类推。 你也可以使用负数下标，以 -1 表示列表的最后一个元素， -2 表示列表的倒数第二个元素，以此类推。

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
        ")
    }

    fn help_lpush(&self) {
        println!("
Redis Lpush 命令将一个或多个值插入到列表头部。 如果 key 不存在，一个空列表会被创建并执行 LPUSH 操作。 当 key 存在但不是列表类型时，返回一个错误。

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
        ")
    }
}
