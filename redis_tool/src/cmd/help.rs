use crate::{
    constrs::constrs,
    util::strexpres::{Express, StrExpress},
};

pub struct Help {
    pub(crate) cmd: String,
}

impl Help {
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
                        "del" => self.help_del(),
                        "dump" => self.help_dump(),
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
【Redis健】
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
        ")
    }
}

pub trait HelpDetail {
    fn help_dump(&self);
    fn help_del(&self);
}

impl HelpDetail for Help {
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
}
