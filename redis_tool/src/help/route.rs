use crate::{
    constrs::constrs,
    util::strexpres::{Express, StrExpress},
};

use super::{
    hash::{HashHelp, Help as HsHelp},
    key::{Help as KHelp, KeyHelp},
    list::{Help as LstHelp, ListHelp},
    set::{Help as StHelp, SetHelp},
    setsort::{Help as SortHelp, SetSortHelp},
    string::{Help as StrHelp, StringHelp},
};

pub struct Route {
    pub(crate) cmd: String,
}

impl Route {
    pub fn new(&self) {
        let items: Vec<&str> = self.cmd.split(" ").collect();
        let usecmds = StrExpress {}.del_null(items);
        match i32::try_from(usecmds.len()) {
            Ok(v) => match v {
                0 => {
                    println!("{}", constrs::CMD_IS_NIL);
                    return;
                }
                1 => {
                    self.main_help();
                    return;
                }
                2 => {
                    let cmdlist = String::from(&usecmds[1]).to_lowercase();
                    match &cmdlist as &str {
                        //help key route
                        "del" => KeyHelp {}.help_del(),
                        "dump" => KeyHelp {}.help_dump(),
                        "exists" => KeyHelp {}.help_exists(),
                        "expire" => KeyHelp {}.help_expire(),
                        "expireat" => KeyHelp {}.help_expireat(),
                        "pexpire" => KeyHelp {}.help_pexpire(),
                        "pexpireat" => KeyHelp {}.help_pexpireat(),
                        "keys" => KeyHelp {}.help_keys(),
                        "move" => KeyHelp {}.help_move(),
                        "persist" => KeyHelp {}.help_persist(),
                        "pttl" => KeyHelp {}.help_pttl(),
                        "ttl" => KeyHelp {}.help_ttl(),
                        "randomkey" => KeyHelp {}.help_randomkey(),
                        "rename" => KeyHelp {}.help_rename(),
                        "renamenx" => KeyHelp {}.help_rename(),
                        "scan" => KeyHelp {}.help_scan(),
                        "type" => KeyHelp {}.help_type(),
                        //help string route
                        "set" => StringHelp {}.help_set(),
                        "get" => StringHelp {}.help_get(),
                        "getrange" => StringHelp {}.help_getrange(),
                        "getset" => StringHelp {}.help_getset(),
                        "getbit" => StringHelp {}.help_getbit(),
                        "mget" => StringHelp {}.help_mget(),
                        "setbit" => StringHelp {}.help_setbit(),
                        "setex" => StringHelp {}.help_setex(),
                        "setnx" => StringHelp {}.help_setnx(),
                        "setrange" => StringHelp {}.help_setrange(),
                        "strlen" => StringHelp {}.help_strlen(),
                        "mset" => StringHelp {}.help_mset(),
                        "msetnx" => StringHelp {}.help_msetnx(),
                        "psetex" => StringHelp {}.help_psetex(),
                        "incr" => StringHelp {}.help_incr(),
                        "incrby" => StringHelp {}.help_incrby(),
                        "decr" => StringHelp {}.help_decr(),
                        "decrby" => StringHelp {}.help_decrby(),
                        "append" => StringHelp {}.help_append(),
                        //help hash route
                        "hdel" => HashHelp {}.help_hdel(),
                        "hexists" => HashHelp {}.help_hexists(),
                        "hget" => HashHelp {}.help_hget(),
                        "hgetall" => HashHelp {}.help_hgetall(),
                        "hincrby" => HashHelp {}.help_hincrby(),
                        "hincrbyfloat" => HashHelp {}.help_hincrbyfloat(),
                        "hkeys" => HashHelp {}.help_hkeys(),
                        "hlen" => HashHelp {}.help_hlen(),
                        "hmget" => HashHelp {}.help_hmget(),
                        "hmset" => HashHelp {}.help_hmset(),
                        "hset" => HashHelp {}.help_hset(),
                        "hsetnx" => HashHelp {}.help_hsetnx(),
                        "hvals" => HashHelp {}.help_hvals(),
                        "hscan" => HashHelp {}.help_hscan(),
                        //help list route
                        "lpush" => ListHelp {}.help_lpush(),
                        "lrange" => ListHelp {}.help_lrange(),
                        "blpop" => ListHelp {}.help_blpop(),
                        "brpop" => ListHelp {}.help_brpop(),
                        "lindex" => ListHelp {}.help_lindex(),
                        "linsert" => ListHelp {}.help_linsert(),
                        "llen" => ListHelp {}.help_llen(),
                        "lpop" => ListHelp {}.help_lpop(),
                        "lpushx" => ListHelp {}.help_lpushx(),
                        "brpoplpush" => ListHelp {}.help_brpoplpush(),
                        "lrem" => ListHelp {}.help_lrem(),
                        "lset" => ListHelp {}.help_lset(),
                        "ltrim" => ListHelp {}.help_ltrim(),
                        "rpop" => ListHelp {}.help_rpop(),
                        "rpoplpush" => ListHelp {}.help_rpoplpush(),
                        "rpush" => ListHelp {}.help_rpush(),
                        "rpushx" => ListHelp {}.help_rpushx(),
                        //help Set route
                        "sadd" => SetHelp {}.help_sadd(),
                        "scard" => SetHelp {}.help_scard(),
                        "sdiff" => SetHelp {}.help_sdiff(),
                        "sdiffstore" => SetHelp {}.help_sdiffstore(),
                        "sinter" => SetHelp {}.help_sinter(),
                        "sinterstore" => SetHelp {}.help_sinterstore(),
                        "sscan" => SetHelp {}.help_sscan(),
                        "sunionstore" => SetHelp {}.help_sunionstore(),
                        "sunion" => SetHelp {}.help_sunion(),
                        "srem" => SetHelp {}.help_srem(),
                        "srandmember" => SetHelp {}.help_srandmember(),
                        "spop" => SetHelp {}.help_spop(),
                        "smove" => SetHelp {}.help_smove(),
                        "smembers" => SetHelp {}.help_smembers(),
                        "sismember" => SetHelp {}.help_sismember(),
                        //help SetSort route
                        "zscan" => SetSortHelp {}.help_zscan(),
                        "zunionstore" => SetSortHelp {}.help_zunionstore(),
                        "zscore" => SetSortHelp {}.help_zscore(),
                        "zrevrange" => SetSortHelp {}.help_zrevrange(),
                        "zremrangebyscore" => SetSortHelp {}.help_zremrangebyscore(),
                        "zremrangebyrank" => SetSortHelp {}.help_zremrangebyrank(),
                        "zremrangebylex" => SetSortHelp {}.help_zremrangebylex(),
                        "zrem" => SetSortHelp {}.help_zrem(),
                        "zrank" => SetSortHelp {}.help_zrank(),
                        "zrangebyscore" => SetSortHelp {}.help_zrangebyscore(),
                        "zrangebylex" => SetSortHelp {}.help_zrangebylex(),
                        "zrange" => SetSortHelp {}.help_zrange(),
                        "zlexcount" => SetSortHelp {}.help_zlexcount(),
                        "zinterstore" => SetSortHelp {}.help_zinterstore(),
                        "zincrby" => SetSortHelp {}.help_zincrby(),
                        "zcount" => SetSortHelp {}.help_zcount(),
                        "zcard" => SetSortHelp {}.help_zcard(),
                        "zadd" => SetSortHelp {}.help_zadd(),
                        "zrevrangebyscore"=>SetSortHelp{}.help_zrevrangebyscore(),
                        "zrevrank"=>SetSortHelp{}.help_zrevrank(),

                        _ => {}
                    }
                }
                _ => {
                    println!("{}", constrs::CMD_IS_FAIL);
                    return;
                }
            },
            Err(error) => {
                println!("{} {}", constrs::CMD_IS_NIL, error);
                return;
            }
        }
    }

    fn main_help(&self) {
        println!("
有关Redis命令使用的详细使用信息
【Key】
DEL               该命令用于在 key 存在时删除 key
DUMP              序列化给定 key ，并返回被序列化的值
EXISTS            检查给定 key 是否存在
EXPIRE            为给定 key 设置过期时间，以秒计
EXPIREAT          作用和 EXPIRE 类似，都用于为 key 设置过期时间,不同在于 EXPIREAT 命令接受的时间参数是 UNIX 时间戳(unix timestamp)
PEXPIRE           设置 key 的过期时间以毫秒计。
PEXPIREAT         设置 key 过期时间的时间戳(unix timestamp) 以毫秒计
KEYS              查找所有符合给定模式( pattern)的 key
MOVE              将当前数据库的 key 移动到给定的数据库 db 当中
PTTL              以毫秒为单位返回 key 的剩余的过期时间
TTL               以秒为单位，返回给定 key 的剩余生存时间(TTL, time to live)
RANDOMKEY         从当前数据库中随机返回一个 key
RENAME            修改 key 的名称
RENAMENX          仅当 newkey 不存在时，将 key 改名为 newkey
SCAN              迭代数据库中的数据库键
TYPE              返回 key 所储存的值的类型

【String】
SET               该命令用于设置给定 key 的值。如果 key 已经存储其他值, SET 就覆写旧值，且无视类型
Get               该命令用于获取指定 key 的值。如果 key 不存在，返回 nil 。如果key 储存的值不是字符串类型，返回一个错误
Getrange          该命令用于获取存储在指定 key 中字符串的子字符串。字符串的截取范围由 start 和 end 两个偏移量决定(包括 start 和 end 在内)
Getset            该命令用于设置指定 key 的值，并返回 key 的旧值
Getbit            该命令用于对 key  所储存的字符串值，获取指定偏移量上的位(bit)
Mget              该命令返回所有(一个或多个)给定 key 的值。 如果给定的 key 里面，有某个 key 不存在，那么这个 key 返回特殊值 nil 
Setbit            该命令用于对 key 所储存的字符串值，设置或清除指定偏移量上的位(bit)
Setex             该命令为指定的 key 设置值及其过期时间。如果 key 已经存在， SETEX 命令将会替换旧的值
Setnx             (SET if Not eXists) 命令在指定的 key 不存在时，为 key 设置指定的值
Setrange          该命令用指定的字符串覆盖给定 key 所储存的字符串值，覆盖的位置从偏移量 offset 开始
Strlen            该命令用于获取指定 key 所储存的字符串值的长度。当 key 储存的不是字符串值时，返回一个错误
Mset              该命令用于同时设置一个或多个 key-value 对
Msetnx            该该命令用于所有给定 key 都不存在时，同时设置一个或多个 key-value 对
Psetex            该命令以毫秒为单位设置 key 的生存时间
Incr              该命令将 key 中储存的数字值增一
Incrby            该命令将 key 中储存的数字加上指定的增量值
Decr              该命令将 key 中储存的数字值减一
Decrby            该命令将 key 所储存的值减去指定的减量值
Append            该命令用于为指定的 key 追加值

【Hash】
Hdel              该命令用于删除哈希表 key 中的一个或多个指定字段，不存在的字段将被忽略
Hexists           该命令用于查看哈希表的指定字段是否存在
Hget              该命令用于返回哈希表中指定字段的值
Hgetall           该命令用于返回哈希表中，所有的字段和值
Hincrby           该命令用于为哈希表中的字段值加上指定增量值
Hincrbyfloat      该命令用于为哈希表中的字段值加上指定浮点数增量值
Hkeys             该命令用于获取哈希表中的所有域（field）
Hlen              该命令用于获取哈希表中字段的数量
Hmget             该命令用于返回哈希表中，一个或多个给定字段的值
Hmset             该命令用于同时将多个 field-value (字段-值)对设置到哈希表中
Hset              该命令用于为哈希表中的字段赋值
Hsetnx            该命令用于为哈希表中不存在的的字段赋值
Hvals             该命令返回哈希表所有的值
HSCAN             该命令用于迭代哈希表中的键值对[无用]

【List】
Rpushx            命令用于将一个值插入到已存在的列表尾部(最右边)。如果列表不存在，操作无效
Rpush             命令用于将一个或多个值插入到列表的尾部(最右边)
Rpoplpush         命令用于移除列表的最后一个元素，并将该元素添加到另一个列表并返回
Rpop              命令用于移除列表的最后一个元素，返回值为移除的元素
Ltrim             对一个列表进行修剪(trim)，就是说，让列表只保留指定区间内的元素，不在指定区间之内的元素都将被删除
Lset              通过索引来设置元素的值。当索引参数超出范围，或对一个空列表进行 LSET 时，返回一个错误
Lrem              根据参数 COUNT 的值，移除列表中与参数 VALUE 相等的元素
Brpoplpush        命令从列表中取出最后一个元素，并插入到另外一个列表的头部;如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止
Lpushx            将一个值插入到已存在的列表头部，列表不存在时操作无效
Lpop              命令用于移除并返回列表的第一个元素
Llen              命令用于返回列表的长度。 如果列表 key 不存在，则 key 被解释为一个空列表，返回 0 。 如果 key 不是列表类型，返回一个错误
Linsert           命令用于在列表的元素前或者后插入元素。当指定元素不存在于列表中时，不执行任何操作
Lindex            命令用于通过索引获取列表中的元素。你也可以使用负数下标，以 -1 表示列表的最后一个元素， -2 表示列表的倒数第二个元素，以此类推
Brpop             命令移出并获取列表的最后一个元素， 如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止
Blpop             命令移出并获取列表的第一个元素， 如果列表没有元素会阻塞列表直到等待超时或发现可弹出元素为止
Lrange            返回列表中指定区间内的元素，区间以偏移量 START 和 END 指定。 其中 0 表示列表的第一个元素，1 表示列表的第二个元素，以此类推。 你也可以使用负数下标，以 -1 表示列表的最后一个元素， -2 表示列表的倒数第二个元素，以此类推
Lpush             命令将一个或多个值插入到列表头部。 如果 key 不存在，一个空列表会被创建并执行 LPUSH 操作。 当 key 存在但不是列表类型时，返回一个错误

【Set】

        ")
    }
}
