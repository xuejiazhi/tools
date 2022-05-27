use std::process::exit;

use prettytable::{Table, Row, Cell};

/**
 * @explain order
 */

#[derive(Debug)]
pub struct Cvt {
    cmd: String,
}

impl Cvt {
    pub fn convert(&self, clients: simple_redis::client::Client) {
        let items: Vec<&str> = self.cmd.split(" ").collect();
        let usecmds = self::Cvt::delnull(items);

        let cmd_length = usecmds.len();

        if cmd_length == 0 {
            println!("{}", crate::constrs::Constrs::CMD_IS_NIL);
            return;
        }
        let cmdlist = String::from(&usecmds[0]).to_lowercase();
        match &cmdlist as &str {
            //get key
            "get" => {
                if cmd_length != 2 {
                    println!("{}", crate::constrs::Constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                let str_val = clients.get::<String>(&usecmds[1]);
                match str_val {
                    Ok(strs) => {
                        // 创建表格
                        let mut table = Table::new();

                        table.add_row(row!["key", "val", "ttl"]);
                        table.add_row(Row::new(vec![
                            Cell::new("key"),
                            Cell::new("val"),
                            Cell::new("-1"),
                        ]));

                        // 打印表格到标准输出
                        table.printstd();
                    }
                    Err(error) => {
                        println!("get error: {}", error);
                    }
                }
                // clients.get(&usecmds[1])
            }
            "set" => {}
            _ => {
                println!("{}", crate::constrs::Constrs::CMD_IS_FAIL);
            }
        }
    }

    fn delnull(cmdlist: Vec<&str>) -> Vec<String> {
        let mut ret = Vec::new();
        if cmdlist.len() > 0 {
            for (_, item) in cmdlist.iter().enumerate() {
                if *item != "" {
                    ret.push(String::from(*item))
                }
            }
        }
        ret
    }
}

// let cmd = String::from(command.trim());
// //match
// match &cmd as &str {
//     "quit" => {
//         println!("quit redis tools");
//         break;
//     }
//     _ => (),
// }
// let str_val = clients.get::<String>(&cmd);
// match str_val {
//     Ok(strs) => {
//         // 创建表格
//         let mut table = Table::new();

//         table.add_row(row!["key", "val", "ttl"]);
//         table.add_row(Row::new(vec![
//             Cell::new(&cmd),
//             Cell::new(&strs),
//             Cell::new("-1"),
//         ]));

//         // 打印表格到标准输出
//         table.printstd();
//     }
//     Err(error) => {
//         println!("get error: {}", error);
//     }
// }
