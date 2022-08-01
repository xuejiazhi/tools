mod cmd;
mod constrs;
mod convert_test;
mod cvt;
mod util;
mod help;

use std::io::{self, stdin, Write};

use regex::Regex;
use util::strparse::StrParse;

use crate::{
    cmd::string::Cmd,
    cvt::Cvt,
    util::strexpres::{Express, StrExpress},
};

extern crate simple_redis;
// include!("cvt.rs");

fn main() -> io::Result<()> {
    //get redis param
    //host port auth
    let mut host = String::new();
    let mut port = String::new();
    let mut auth = String::new();
    parse_args(&mut host, &mut port, &mut auth);
    let mut parmas = &mut RedisParams {
        host: host,
        port: port,
        db: String::from("0"),
        auth: auth,
    };

    let clients = &mut parmas.new();
    match clients.echo("Ping") {
        Ok(_) => {
            println!(
                "  
    .-\"\"\"-.
    / .===. \\
    \\/ 6 6 \\/
    ( \\___/ )
 _________ooo__\\_____/_____________
/                                  \\
|    Connect Redis Success!         |
\\_______________________ooo________/ 
     |  |  |
     |_ | _|
     |  |  |
     |__|__|
     /-'Y'-\\
    (__/ \\__)"
            );
        }
        Err(_) => {
            println!("Connect Refused,Please Check the NetWork ");
            return Ok(());
        }
    }

    loop {
        print!("{}:{}~[db{}]#> ", &parmas.host, &parmas.port, &parmas.db);

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
                        println!("Bye! quit redis tools");
                        break;
                    }
                    "clear" => {
                        cmd::string::StringCMD {}.clear();
                        print!("\x1b[2J");
                        print!("\x1b[H");
                        continue;
                    }
                    _ => {
                        let r = Regex::new(r"db([0-9]\b|1[0-5]\b)").unwrap();
                        if r.is_match(cmd.as_str()) {
                            //switch redis db
                            //use db{0-15}
                            //like [ # > db1 | # > db2 .......# > db15]
                            parmas.db =
                                StrExpress {}.replace(&cmd, "db".to_string(), "".to_string());
                            println!("switch db {} Success!", &parmas.db);
                            *clients = parmas.new();
                            continue;
                        }
                        // let clients11 = &mut clients;
                        Cvt {
                            cmd,
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

fn parse_args(host: &mut String, port: &mut String, auth: &mut String) {
    let sc = StrParse::new();
    sc.to_string(host, "-h", "10.161.55.194");
    sc.to_string(port, "-p", "6379");
    sc.to_string(auth, "-a", "");
}
