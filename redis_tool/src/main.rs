#[macro_use] extern crate prettytable;
use std::io::{self, Write};

use prettytable::{Table, Cell, Row, row};
// use simple_redis::{client, RedisError};
extern crate simple_redis;

fn main() -> io::Result<()> {
    let parmas = RedisParams {
        host: String::from("127.0.0.1"),
        port: String::from("6379"),
        db: 0,
        auth: String::from(""),
    };

    let mut clients = parmas.new();

    loop {
        print!("#>");
        io::stdout().flush()?;
        let mut command = String::new();
        let b1 = std::io::stdin().read_line(&mut command).unwrap();
        let cmd = String::from(command.trim());
        match &cmd as &str {
            "quit" => {
                println!("quit redis tools");
                break;
            },
            _ => (),
        }
        let str_val = clients.get::<String>(&cmd);
        match str_val {
            Ok(mut strs) => {
               // 创建表格
               let mut table = Table::new();

               table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
               table.add_row(row!["foobar", "bar", "foo"]);
               table.add_row(Row::new(vec![
               Cell::new("foobar2"),
               Cell::new("bar2"),
               Cell::new("foo2"),
               ]));

             // 打印表格到标准输出
            table.printstd();
                println!("val=>{}",&strs);
            },
            Err(error) =>{
                println!("get error: {}",error);
            } ,
        }
        // println!("val=>{}", &str_val);
    }

    Ok(())
    // let str_val = clients.get::<String>("test").unwrap();
    // println!("v=>{}",str_val);
    // match &str_val {
    //     Ok(mut str) => {
    //         println!("value=>{}", str)
    //     }
    //     Err(error) => {
    //         println!("Get Key {} Error：{}", "Affa_Sub_32896668190032845", error)
    //     }
    // }
}

#[derive(Debug)]
struct RedisParams {
    host: String,
    port: String,
    db: u16,
    auth: String,
}


impl RedisParams {
    //connect redis server
    pub fn new(&self) -> simple_redis::client::Client {
        let s = format!("redis://{}:{}/{}", self.host, self.port, self.db);
        match simple_redis::create(&s) {
            Ok(mut client) => {
                if !String::from(&self.auth).is_empty() {
                    match client.auth(&self.auth) {
                        Err(error) => {
                            panic!("Auth error: {}", error)
                        }
                        _ => {
                            println!("Authenticated");
                            client
                        }
                    }
                } else {
                    client
                }
            }
            Err(error) => panic!("Unable to create Redis client: {}", error),
        }
    }
}
