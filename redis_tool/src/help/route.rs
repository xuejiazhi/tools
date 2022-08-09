use crate::{
    constrs::constrs,
    util::strexpres::{Express, StrExpress},
};

use super::{
    hash::{HashHelp, Help as HsHelp},
    key::{Help as KHelp, KeyHelp},
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
                        "hscan" => HashHelp{}.help_hscan(),
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
        ")
    }
}
