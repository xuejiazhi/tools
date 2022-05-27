mod constrs;
mod cvt;

#[macro_use]
extern crate prettytable;
use std::{
    clone,
    io::{self, stdin, Write},
};

// use prettytable::{row, Cell, Row, Table};
// use simple_redis::{client, RedisError};
extern crate simple_redis;
include!("cvt.rs");

fn main() -> io::Result<()> {
    let parmas = RedisParams {
        host: String::from("127.0.0.1"),
        port: String::from("6379"),
        db: 0,
        auth: String::from(""),
    };

    let mut clients = parmas.new();

    loop {
        print!("#_>");

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
                        let mut clients11 = clients;
                        Cvt { cmd: cmd }.convert(clients11);
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
