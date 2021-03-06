use crate::{
    cmd as cvt_cmd,
    cmd::string::Cmd,
    constrs::constrs,
    util::{
        strexpres::{Express, StrExpress},
        tagregs::{Regs, TagRegs},
    },
};

use std::collections::HashMap;

/**
 * @explain order
 */
pub struct Cvt {
    pub(crate) cmd: String,
    pub(crate) clients: *mut simple_redis::client::Client,
}

impl Cvt {
    #[allow(dead_code)]
    pub fn convert(&self) {
        let tag = TagRegs {}.reg_match_quotation(self.cmd.clone());
        // let items: Vec<&str> = self.cmd.split(" ").collect();
        let items: Vec<&str> = tag.split(" ").collect();
        let usecmds = StrExpress {}.del_null(items);
        // .del_null(items);

        let cmd_length = usecmds.len();
        if cmd_length == 0 {
            println!("{}", constrs::CMD_IS_NIL);
            return;
        }

        //match cmd to oprate
        let cmdlist = String::from(&usecmds[0]).to_lowercase();
        match &cmdlist as &str {
            "help" => cvt_cmd::help::Help {
                cmd: self.cmd.to_string(),
            }
            .new(),
            // @ Redis Key Operate
            // like <del,dump,keys,type,exists,pttl,ttl>
            // Keys used to manage redis
            // [del]  This command is used to delete a key when it exists.
            "del" => {
                if cmd_length != 2 {
                    println!("DEL {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.del(usecmds[1].to_string()) }
            }

            //[dump] Serialize the given key and return the serialized value.
            "dump" => {
                if cmd_length != 2 {
                    println!("DUMP {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.dump(usecmds[1].to_string());
                }
            }

            //[keys] Find all keys that match the given pattern.
            "keys" => {
                if cmd_length != 2 {
                    println!("KEYS {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.keys(usecmds[1].to_string()) }
            }

            //[type] Returns the type of the value stored by the key.
            "type" => {
                if cmd_length != 2 {
                    println!("TYPE {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    let param_type = self.get_type(usecmds[1].to_string());
                    cvt_cmd::string::StringCMD {}.typec(usecmds[1].to_string(), param_type)
                }
            }

            //[exists] Check whether the given key exists.
            "exists" => {
                if cmd_length != 2 {
                    println!("EXISTS {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { println!("{}", self.key_is_exists(usecmds[1].to_string())) }
            }

            //[pttl]
            //Returns the remaining expiration time of the key in milliseconds
            "pttl" => {
                if cmd_length != 2 {
                    println!("PTTL {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.pttl_key(usecmds[1].to_string());
                }
            }

            //[ttl]
            //Returns the remaining expiration time of the key in seconds
            "ttl" => {
                if cmd_length != 2 {
                    println!("TTL {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.ttl_key(usecmds[1].to_string());
                }
            }

            //[expire]
            //Sets the expiration time in seconds for the given key.
            "expire" => {
                if cmd_length != 3 {
                    println!("EXPIRE {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                match usize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(ret) => unsafe {
                        self.expire(usecmds[1].to_string(), ret);
                    },
                    Err(error) => {
                        println!("{}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[pexpire]
            "pexpire" => {
                if cmd_length != 3 {
                    println!("PEXPIRE {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                match usize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(ret) => unsafe {
                        self.pexpire(usecmds[1].to_string(), ret);
                    },
                    Err(error) => {
                        println!("{}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[Expireat]
            "expireat" => {
                if cmd_length != 3 {
                    println!("Expireat {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                match usize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(ret) => unsafe {
                        self.expireat(usecmds[1].to_string(), ret.to_string());
                    },
                    Err(error) => {
                        println!("{}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[PExpireat]
            "pexpireat" => {
                if cmd_length != 3 {
                    println!("PExpireat {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                match usize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(ret) => unsafe {
                        self.pexpireat(usecmds[1].to_string(), ret.to_string());
                    },
                    Err(error) => {
                        println!("{}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[MOVE]
            "move" => {
                if cmd_length != 3 {
                    println!("MOVE {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.movedb(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //[PERSIST]
            "persist" => {
                if cmd_length != 2 {
                    println!("PERSIST {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.persist(usecmds[1].to_string());
                }
            }

            //[RANDOMKEY]
            "randomkey" => {
                if cmd_length != 1 {
                    println!("RANDOMKEY {}", constrs::CLI_PARAM_IS_FAIL);
                    return;
                }
                unsafe {
                    self.randomkey();
                }
            }

            //[RENAME]
            "rename" => {
                if cmd_length != 3 {
                    println!("RENAME {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.rename(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //[RENAMENX]
            "renamenx" => {
                if cmd_length != 3 {
                    println!("RENAMENX {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.rename(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //[SCAN]
            "scan" => {
                if cmd_length != 2 {
                    println!("SCAN {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                match usize::from_str_radix(usecmds[1].as_str(), 10) {
                    Ok(_) => unsafe {
                        self.scan(usecmds[1].to_string());
                    },
                    Err(error) => {
                        println!("{}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }
            // @ Redis String Operate
            // like <del,dump,keys,type,exists,pttl>
            // Commands related to redis string data types are used to manage redis string values
            // [get] The get command is used to get the value of the specified key.
            // If the key does not exist, nil is returned.
            // If the value stored by key is not of string type, an error is returned
            "get" => {
                if cmd_length != 2 {
                    println!("GET {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                unsafe {
                    self.get(usecmds[1].to_string());
                }
            }

            //[set]
            // The set command is used to set the value of a given key.
            // If the key already stores other values, set overwrites the old values and ignores the type
            "set" => {
                if cmd_length != 3 {
                    println!("SET {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.set(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //getset
            "getset" => {
                if cmd_length != 3 {
                    println!("GETSET {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                unsafe {
                    self.get(usecmds[1].to_string());
                    self.set(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //GETRANGE
            "getrange" => {
                if cmd_length != 4 {
                    println!("GETRANGE {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    match usize::from_str_radix(usecmds[2].as_str(), 10) {
                        Ok(x) => match usize::from_str_radix(usecmds[3].as_str(), 10) {
                            Ok(y) => {
                                self.getrange(usecmds[1].to_string(), x.to_string(), y.to_string());
                            }
                            Err(error) => {
                                println!("{}{}", constrs::CMD_IS_FAIL, error);
                                return;
                            }
                        },
                        Err(error) => {
                            println!("{}{}", constrs::CMD_IS_FAIL, error);
                            return;
                        }
                    }
                }
            }
            //GETBIT
            "getbit" => {
                if cmd_length != 3 {
                    println!("GETBIT {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    match usize::from_str_radix(usecmds[2].as_str(), 10) {
                        Ok(_) => self.getbit(usecmds[1].to_string(), usecmds[2].to_string()),
                        Err(error) => {
                            println!("{}{}", constrs::CMD_IS_FAIL, error);
                            return;
                        }
                    }
                }
            }

            //GETBIT
            "setbit" => {
                if cmd_length != 4 {
                    println!("SETBIT {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    match usize::from_str_radix(usecmds[2].as_str(), 10) {
                        Ok(_) => match usize::from_str_radix(usecmds[3].as_str(), 10) {
                            Ok(_) => self.setbit(
                                usecmds[1].to_string(),
                                usecmds[2].to_string(),
                                usecmds[3].to_string(),
                            ),
                            Err(error) => {
                                println!("{}{}", constrs::CMD_IS_FAIL, error);
                                return;
                            }
                        },
                        Err(error) => {
                            println!("{}{}", constrs::CMD_IS_FAIL, error);
                            return;
                        }
                    }
                }
            }

            //[MGET]
            "mget" => {
                if cmd_length < 2 {
                    println!("MGET {} 2", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.mget(vec) }
            }

            //[SETEX]
            "setex" => {
                if cmd_length != 4 {
                    println!("SETEX {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                let timeout = usize::from_str_radix(usecmds[2].as_str(), 10);
                match timeout {
                    Ok(t) => unsafe {
                        self.setex(usecmds[1].to_string(), t, usecmds[3].to_string());
                    },
                    Err(error) => {
                        println!("timeout must numeric {}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[SETNX]
            "setnx" => {
                if cmd_length != 3 {
                    println!("SETNX {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.set(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //[SETRANGE]
            "setrange" => {
                if cmd_length != 4 {
                    println!("SETRANGE {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                let offset = usize::from_str_radix(usecmds[2].as_str(), 10);
                match offset {
                    Ok(t) => unsafe {
                        self.setex(usecmds[1].to_string(), t, usecmds[3].to_string());
                    },
                    Err(error) => {
                        println!("offset must numeric {}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }
            //[strlen]
            "strlen" => {
                if cmd_length != 2 {
                    println!("STRLEN {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.strlen(usecmds[1].to_string());
                }
            }

            _ => {
                println!("{}", constrs::CMD_IS_FAIL);
            }
        }
    }
}

pub trait RunUnsafe {
    //string
    unsafe fn strlen(&self, key: String);
    unsafe fn setrange(&self, key: String, offset: String, value: String);
    unsafe fn setnx(&self, key: String, value: String);
    unsafe fn setex(&self, key: String, timeout: usize, value: String);
    unsafe fn mget(&self, keys: Vec<&str>);
    unsafe fn setbit(&self, key: String, offset: String, value: String);
    unsafe fn getbit(&self, key: String, offset: String);
    unsafe fn getrange(&self, key: String, start: String, end: String);
    unsafe fn get(&self, key: String);
    unsafe fn set(&self, key: String, value: String);
    //keys
    unsafe fn scan(&self, cursor: String);
    unsafe fn renamenx(&self, oldkey: String, newkey: String);
    unsafe fn rename(&self, oldkey: String, newkey: String);
    unsafe fn randomkey(&self);
    unsafe fn persist(&self, key: String);
    unsafe fn movedb(&self, key: String, db: String);
    unsafe fn pexpireat(&self, key: String, miltimestamp: String);
    unsafe fn expireat(&self, key: String, timestamp: String);
    unsafe fn pexpire(&self, key: String, millseconds: usize);
    unsafe fn expire(&self, key: String, seconds: usize);
    unsafe fn dump(&self, key: String);
    unsafe fn keys(&self, key: String);
    unsafe fn del(&self, key: String);
    unsafe fn get_type(&self, key: String) -> String;
    unsafe fn pttl_key(&self, key: String);
    unsafe fn ttl_key(&self, key: String);
    unsafe fn key_is_exists(&self, key: String) -> bool;
    unsafe fn get_ttl(&self, key: String) -> (String, String);
}

impl RunUnsafe for Cvt {
    #[allow(dead_code)]
    unsafe fn strlen(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.strlen(&key) {
            Ok(v) => {
                cvt_cmd::string::StringCMD {}.strlen(key, v);
            }
            Err(error) => {
                println!("strlen error {}", error.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn setrange(&self, key: String, offset: String, value: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<i32>("SETRANGE", vec![&key, &offset, &value]) {
            Ok(i) => {
                println!("setrange {} success, {}", key.clone(), i);
                self.get(key)
            }
            Err(error) => println!("Unable to setrange value in Redis: {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn setnx(&self, key: String, value: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.setnx(&key, value.as_str()) {
            Ok(_) => {
                cvt_cmd::string::StringCMD {}.set(
                    key.to_string(),
                    value.to_string(),
                    "nil".to_string(),
                );
            }
            Err(_error) => {
                cvt_cmd::string::StringCMD {}.set(
                    key.to_string(),
                    value.to_string(),
                    _error.to_string(),
                );
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn setex(&self, key: String, timeout: usize, value: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.setex(&key, value.as_str(), timeout) {
            Err(error) => println!("Unable to setex value in Redis: {}", error),
            _ => {
                println!("setex success");
                self.get(key)
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn mget(&self, keys: Vec<&str>) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("MGET", keys.clone()) {
            Ok(v) => {
                let mut map_data = HashMap::new();
                for k in 0..keys.len() {
                    map_data.insert(keys[k].to_string(), v[k].to_string());
                }
                cvt_cmd::string::StringCMD {}.mget(map_data);
            }
            Err(_) => todo!(),
        }
    }

    #[allow(dead_code)]
    unsafe fn setbit(&self, key: String, offset: String, value: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<i32>("SETBIT", vec![&key, &offset, &value]) {
            Ok(v) => {
                println!("setbit (integer) {}", v);
            }
            Err(error) => {
                println!("setbit error {}", error);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn getbit(&self, key: String, offset: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<i32>("GETBIT", vec![&key, &offset]) {
            Ok(v) => {
                println!("getbit (integer) {}", v);
            }
            Err(error) => {
                println!("getbit error {}", error);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn getrange(&self, key: String, start: String, end: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<String>("GETRANGE", vec![&key, &start, &end]) {
            Ok(v) => {
                println!("{}", v)
            }
            Err(error) => {
                println!("GETRANGE is Fail,Msg is {}", error)
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn scan(&self, cursor: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<HashMap<String, Vec<String>>>("SCAN", vec![&cursor]) {
            Ok(val) => {
                for (k, v) in val {
                    println!("Cursor -> {}", k);
                    cvt_cmd::string::StringCMD {}.keys(v);
                }
            }
            Err(error) => {
                println!("RENAMENX Cli Is Failed!,Error {}", error);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn renamenx(&self, oldkey: String, newkey: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.renamenx(&oldkey, &newkey) {
            Ok(_) => {
                println!(
                    "RENAMENX Success Old key <{}>, New Key <{}>",
                    oldkey,
                    newkey.clone()
                );
                self.get(newkey)
            }
            Err(error) => {
                println!("RENAMENX Cli Is Failed!,Error {}", error);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn rename(&self, oldkey: String, newkey: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.rename(&oldkey, &newkey) {
            Ok(_) => {
                println!(
                    "RENAME Success Old key <{}>, New Key <{}>",
                    oldkey,
                    newkey.clone()
                );
                self.get(newkey)
            }
            Err(error) => {
                println!("RENAME Cli Is Failed!,Error {}", error);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn randomkey(&self) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<String>("RANDOMKEY", vec![]) {
            Ok(v) => {
                let param_type = self.get_type(v.clone());
                let (ttl, _) = self.get_ttl(v.clone());
                cvt_cmd::string::StringCMD {}.randomkey(v, param_type, ttl)
            }
            Err(_) => {
                println!("RANDOMKEY Cli Not Get Key!");
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn persist(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.persist(&key) {
            Ok(_) => {
                println!("Persist Key {} Success", key);
                self.get(key)
            }
            Err(error) => {
                println!("Move fail! Error {}", error);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn movedb(&self, key: String, db: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command_empty_response("MOVE", vec![&key, &db]) {
            Ok(_) => {
                println!("Move Key {} success!", key);
            }
            Err(_) => {
                println!("Move fail!");
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn pexpireat(&self, key: String, miltimestamp: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command_empty_response("PEXPIREAT", vec![&key, &miltimestamp]) {
            Ok(_) => {
                println!("PExpireat success");
                self.get(key);
            }
            Err(_) => {
                println!("PExpireat fail!")
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn expireat(&self, key: String, timestamp: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command_empty_response("EXPIREAT", vec![&key, &timestamp]) {
            Ok(_) => {
                println!("Expireat success");
                self.get(key);
            }
            Err(_) => {
                println!("Expireat fail!")
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn pexpire(&self, key: String, millseconds: usize) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.pexpire(&key, millseconds) {
            Ok(_) => {
                println!("PExpire success");
                self.get(key);
            }
            Err(_) => {
                println!("PExpire fail!")
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn expire(&self, key: String, seconds: usize) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.expire(&key, seconds) {
            Ok(_) => {
                println!("Expire success");
                self.get(key);
            }
            Err(_) => {
                println!("Expire fail!")
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn dump(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<Vec<u8>>("DUMP", vec![&key]) {
            Ok(val) => {
                let str = String::from_utf8_lossy(&val);
                println!("{}", str)
            }
            Err(_error) => {}
        }
    }

    #[allow(dead_code)]
    unsafe fn keys(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.keys(&key) {
            Ok(strs) => {
                cvt_cmd::string::StringCMD {}.keys(strs);
            }
            Err(error) => {
                println!("get error: {}", error);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn set(&self, key: String, value: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.set(key.as_str(), value.as_str()) {
            Ok(_) => {
                cvt_cmd::string::StringCMD {}.set(
                    key.to_string(),
                    value.to_string(),
                    "nil".to_string(),
                );
            }
            Err(_error) => {
                cvt_cmd::string::StringCMD {}.set(
                    key.to_string(),
                    value.to_string(),
                    _error.to_string(),
                );
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn del(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.del(key.as_str()) {
            Ok(_) => {
                cvt_cmd::string::StringCMD {}.opt(
                    "DEL".to_string(),
                    key.to_string(),
                    "nil".to_string(),
                );
            }
            Err(_error) => {
                cvt_cmd::string::StringCMD {}.opt(
                    "DEL".to_string(),
                    key.to_string(),
                    _error.to_string(),
                );
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn get_type(&self, key: String) -> String {
        //set ttl error
        let mut type_val: String = "none".to_string();
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<String>("TYPE", vec![&key]) {
            Ok(val) => {
                type_val = val.to_string();
            }
            Err(_error) => {}
        }
        type_val
    }

    #[allow(dead_code)]
    unsafe fn key_is_exists(&self, key: String) -> bool {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.exists(&key) {
            Ok(result) => result,
            Err(_) => false,
        }
    }

    #[allow(dead_code)]
    unsafe fn ttl_key(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<i32>("TTL", vec![&key]) {
            Ok(val) => {
                println!("{} <seconds>", val);
            }
            Err(_) => {
                println!("NIL")
            }
        };
    }

    #[allow(dead_code)]
    unsafe fn pttl_key(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<i32>("PTTL", vec![&key]) {
            Ok(val) => {
                println!("{} <milliseconds>", val);
            }
            Err(_) => {
                println!("NIL")
            }
        };
    }

    #[allow(dead_code)]
    unsafe fn get(&self, key: String) {
        let (ttl, err) = self.get_ttl(key.to_string());
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        let str_val = c.get::<String>(&key);
        match str_val {
            Ok(strs) => {
                cvt_cmd::string::StringCMD {}.get(key.to_string(), strs.to_string(), ttl, err);
            }
            Err(error) => {
                println!("get error: {}", error);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn get_ttl(&self, key: String) -> (String, String) {
        //set ttl error
        let mut ttl_val: String = "nil".to_string();
        let mut err_val: String = "nil".to_string();
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<i32>("TTL", vec![&key]) {
            Ok(val) => {
                ttl_val = val.to_string();
            }
            Err(err) => {
                err_val = err.to_string();
            }
        };
        (ttl_val, err_val)
    }
}
