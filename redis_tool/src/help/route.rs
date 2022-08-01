use crate::{
    constrs::constrs,
    util::strexpres::{Express, StrExpress},
};

use super::key::{Help, KeyHelp};

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
                        "expireat" => KeyHelp{}.help_expireat(),
                        "pexpire" => KeyHelp{}.help_pexpire(),
                        "pexpireat" => KeyHelp{}.help_pexpireat(),
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
