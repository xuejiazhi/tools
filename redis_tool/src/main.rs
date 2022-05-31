mod cmd;
mod constrs;
mod cvt;
mod util;

use std::io::{self, stdin, Write};

use regex::Regex;

use crate::util::strexpres::{Express, StrExpress};

extern crate simple_redis;
include!("cvt.rs");

fn main() -> io::Result<()> {
    let mut parmas = &mut RedisParams {
        host: String::from("127.0.0.1"),
        port: String::from("6379"),
        db: String::from("0"),
        auth: String::from(""),
    };

    let mut clients = &mut parmas.new();

    loop {
        print!("#_> ");

        //flush std io
        //set params from readline
        //read line from keyborad
        io::stdout().flush()?;
        let mut command = String::new();

        match stdin().read_line(&mut command) {
            Ok(_) => {
                let cmd = String::from(command.trim());
                //match
                match &cmd as &str {
                    "quit" => {
                        println!("quit redis tools");
                        break;
                    }
                    _ => {
                        let r = Regex::new(r"db([0-9]\b|1[0-5]\b)").unwrap();
                        if r.is_match(cmd.as_str()) {
                            //switch redis db
                            //use db{0-15}
                            //like [ # > db1 | # > db2 .......# > db15]
                            parmas.db =
                                StrExpress {}.replace(&cmd, "db".to_string(), "".to_string());
                            println!("switch {:?} db", &parmas);
                            *clients = parmas.new();
                            continue;
                        }
                        // let clients11 = &mut clients;
                        Cvt {
                            cmd: cmd,
                            clients: clients,
                        }
                        .convert();
                    }
                }
            }
            Err(error) => {
                print!("read cmd line is error,err {}", error)
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
struct RedisParams {
    host: String,
    port: String,
    db: String,
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
