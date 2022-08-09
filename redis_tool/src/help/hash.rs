#[derive(Debug, Clone)]
pub struct HashHelp {}

pub trait Help {
    fn help_hdel(&self);
    fn help_hexists(&self);
    fn help_hget(&self);
    fn help_hgetall(&self);
    fn help_hincrby(&self);
    fn help_hincrbyfloat(&self);
    fn help_hkeys(&self);
    fn help_hlen(&self);
    fn help_hmget(&self);
    fn help_hmset(&self);
    fn help_hset(&self);
    fn help_hsetnx(&self);
    fn help_hvals(&self);
    fn help_hscan(&self);
}

impl Help for HashHelp {
    fn help_hscan(&self) {
        println!(
            "
Redis HSCAN 命令用于迭代哈希表中的键值对。

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
127.0.0.1:6379~[db0]#> hscan mykey 0 match 10
hscan->['0']
        "
        )
    }

    fn help_hvals(&self) {
        println!(
            "
Redis Hvals 命令返回哈希表所有的值。

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
    "
        )
    }

    fn help_hsetnx(&self) {
        println!(
            "
Redis Hsetnx 命令用于为哈希表中不存在的的字段赋值 。

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
        "
        )
    }

    fn help_hset(&self) {
        println!("
Redis Hset 命令用于为哈希表中的字段赋值 。

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
    ")
    }

    fn help_hmset(&self) {
        println!(
            "
Redis Hmset 命令用于同时将多个 field-value (字段-值)对设置到哈希表中。

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
    
    "
        )
    }

    fn help_hmget(&self) {
        println!(
            "
Redis Hmget 命令用于返回哈希表中，一个或多个给定字段的值。

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
    "
        )
    }

    fn help_hlen(&self) {
        println!(
            "
Redis Hlen 命令用于获取哈希表中字段的数量。

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
    "
        )
    }

    fn help_hkeys(&self) {
        println!(
            "
Redis Hkeys 命令用于获取哈希表中的所有域（field）。

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
    "
        )
    }

    fn help_hincrbyfloat(&self) {
        println!(
            "
Redis Hincrbyfloat 命令用于为哈希表中的字段值加上指定浮点数增量值。

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
    "
        )
    }

    fn help_hincrby(&self) {
        println!(
            "
Redis Hincrby 命令用于为哈希表中的字段值加上指定增量值。

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
        "
        )
    }

    fn help_hgetall(&self) {
        println!(
            "  
Redis Hgetall 命令用于返回哈希表中，所有的字段和值。

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
    "
        )
    }

    fn help_hget(&self) {
        println!(
            "
Redis Hget 命令用于返回哈希表中指定字段的值。

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
        "
        )
    }

    fn help_hexists(&self) {
        println!(
            "
Redis Hexists 命令用于查看哈希表的指定字段是否存在。

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
        "
        )
    }

    fn help_hdel(&self) {
        println!(
            "
Redis Hdel 命令用于删除哈希表 key 中的一个或多个指定字段，不存在的字段将被忽略。

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
        "
        )
    }
}
