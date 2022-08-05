use crate::{
    cmd as cvt_cmd,
    cmd::{hash::Cmd as HashCmd, list::Cmd as ListCmd, string::Cmd},
    constrs::constrs,
    help::route::Route,
    util::{
        function,
        strexpres::{Express, StrExpress},
        tagregs::{Regs, TagRegs},
    },
};

use std::{collections::HashMap};

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
        let tag = TagRegs {}.reg_captures(self.cmd.clone());
        // println!("tag->{}",tag.clone());
        let items: Vec<&str> = tag.split(" ").collect();
        let usecmds = StrExpress {}.del_null(items);
        // println!("{:?}",usecmds.clone());

        let cmd_length = usecmds.len();
        if cmd_length == 0 {
            println!("{}", constrs::CMD_IS_NIL);
            return;
        }

        //match cmd to oprate
        let cmdlist = String::from(&usecmds[0]).to_lowercase();
        match &cmdlist as &str {
            "help" => Route {
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
                    self.setnx(usecmds[1].to_string(), usecmds[2].to_string());
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
                    Ok(_) => unsafe {
                        self.setrange(usecmds[1].to_string(), usecmds[2].to_string(), usecmds[3].to_string());
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

            "mset" => {
                if cmd_length < 3 {
                    println!("MSET {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.mset(vec) }
            }

            "msetnx" => {
                if cmd_length < 3 {
                    println!("MSETNX {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.msetnx(vec) }
            }

            //[PSETEX]
            "psetex" => {
                if cmd_length != 4 {
                    println!("PSETEX {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                let timeout = usize::from_str_radix(usecmds[2].as_str(), 10);
                match timeout {
                    Ok(t) => unsafe {
                        self.psetex(usecmds[1].to_string(), t, usecmds[3].to_string());
                    },
                    Err(error) => {
                        println!("timeout must numeric {}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[INCR]
            "incr" => {
                if cmd_length != 2 {
                    println!("INCR {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                unsafe {
                    self.incr(usecmds[1].to_string());
                }
            }

            //[INCRBY]
            "incrby" => {
                if cmd_length != 3 {
                    println!("INCRBY {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                let t = usize::from_str_radix(usecmds[2].as_str(), 10);
                match t {
                    Ok(_) => unsafe {
                        self.incrby(usecmds[1].to_string(), usecmds[2].to_string());
                    },
                    Err(error) => {
                        println!(
                            "incrby amount must numeric {}{}",
                            constrs::CMD_IS_FAIL,
                            error
                        );
                        return;
                    }
                }
            }

            "incrbyfloat" => {
                if cmd_length != 3 {
                    println!("INCRBYFLOAT {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                match usecmds[2].as_str().parse::<f64>() {
                    Ok(_) => unsafe {
                        self.incrbyfloat(usecmds[1].to_string(), usecmds[2].to_string())
                    },
                    Err(e) => {
                        println!("error=>{}", e.to_string());
                        return;
                    }
                }
            }

            //[DECR]
            "decr" => {
                if cmd_length != 2 {
                    println!("DECR {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.decr(usecmds[1].to_string());
                }
            }

            //[DECRBY]
            "decrby" => {
                if cmd_length != 3 {
                    println!("DECRBY {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                //judge
                let t = usize::from_str_radix(usecmds[2].as_str(), 10);
                match t {
                    Ok(_) => unsafe {
                        self.decrby(usecmds[1].to_string(), usecmds[2].to_string());
                    },
                    Err(error) => {
                        println!(
                            "decrby amount must numeric {}{}",
                            constrs::CMD_IS_FAIL,
                            error
                        );
                        return;
                    }
                }
            }

            "append" => {
                if cmd_length != 3 {
                    println!("APPEND {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.append(usecmds[1].to_string(), usecmds[2].to_string()) }
            }

            /* HASH  操作 */
            //[HSET]
            "hset" => {
                if cmd_length != 4 {
                    println!("HSET {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.hset(
                        usecmds[1].to_string(),
                        usecmds[2].to_string(),
                        usecmds[3].to_string(),
                    )
                }
            }

            "hget" => {
                if cmd_length != 3 {
                    println!("HGET {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.hget(usecmds[1].to_string(), usecmds[2].to_string()) }
            }

            "hdel" => {
                if cmd_length < 3 {
                    println!("HDEL {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                unsafe { self.hdel(usecmds[1].to_string(), usecmds[2].to_string()) }
            }

            //[HEXISTS]
            "hexists" => {
                if cmd_length != 3 {
                    println!("HEXISTS {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.hexists(usecmds[1].to_string(), usecmds[2].to_string()) }
            }

            "hgetall" => {
                if cmd_length != 2 {
                    println!("HGETALL {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.hgetall(usecmds[1].to_string()) }
            }

            //[HINCRBY]
            "hincrby" => {
                if cmd_length != 4 {
                    println!("HINCRBY {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                let t = usize::from_str_radix(usecmds[3].as_str(), 10);
                match t {
                    Ok(_) => unsafe {
                        self.hincrby(
                            usecmds[1].to_string(),
                            usecmds[2].to_string(),
                            usecmds[3].to_string(),
                        )
                    },
                    Err(error) => {
                        println!(
                            "hincrby incrby amount must numeric {}{}",
                            constrs::CMD_IS_FAIL,
                            error
                        );
                        return;
                    }
                }
            }

            //[HINCRBYFLOAT]
            "hincrbyfloat" => {
                if cmd_length != 4 {
                    println!("HINCRBYFLOAT {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                match usecmds[3].as_str().parse::<f64>() {
                    Ok(_) => unsafe {
                        self.hincrbyfloat(
                            usecmds[1].to_string(),
                            usecmds[2].to_string(),
                            usecmds[3].to_string(),
                        )
                    },
                    Err(e) => {
                        println!("error=>{}", e.to_string());
                        return;
                    }
                }
            }

            "hkeys" => {
                if cmd_length != 2 {
                    println!("HKEYS {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                unsafe { self.hkeys(usecmds[1].to_string()) }
            }

            "hlen" => {
                if cmd_length != 2 {
                    println!("HLEN {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                unsafe { self.hlen(usecmds[1].to_string()) }
            }

            //[HMGET]
            "hmget" => {
                if cmd_length < 3 {
                    println!("HMGET {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.hmget(vec) }
            }

            //[HMSET]
            "hmset" => {
                if cmd_length < 4 {
                    println!("HMSET {} 4", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.hmset(vec) }
            }

            //[HSETNX]
            "hsetnx" => {
                if cmd_length != 4 {
                    println!("HSETNX {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.hsetnx(
                        usecmds[1].to_string(),
                        usecmds[2].to_string(),
                        usecmds[3].to_string(),
                    )
                }
            }

            //[HVALS]
            "hvals" => {
                if cmd_length != 2 {
                    println!("HVALS {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.hvals(usecmds[1].to_string()) }
            }

            //[HSCAN]
            "hscan" => {
                if cmd_length < 5 || cmd_length > 7 {
                    println!("HSCAN length is Failed");
                    return;
                }
                //判断cursor
                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.hscan(vec) }
            }

            // redis list 列表相关命令的实现
            // Redis列表是简单的字符串列表，按照插入顺序排序。你可以添加一个元素到列表的头部（左边）或者尾部（右边）
            // 一个列表最多可以包含 232 - 1 个元素 (4294967295, 每个列表超过40亿个元素)

            //[LPUSH]
            "lpush" => {
                if cmd_length < 3 {
                    println!("LPUSH {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 2..usecmds.len() {
                    vec.push(&usecmds[i])
                }

                unsafe { self.lpush(usecmds[1].to_string(), vec) }
            }
            //[BLPOP]
            "blpop" => {
                if cmd_length < 3 {
                    println!("BLPOP {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }

                match usize::from_str_radix(usecmds[&usecmds.len() - 1].as_str(), 10) {
                    Ok(_) => unsafe {
                        self.blpop(vec);
                    },
                    Err(error) => {
                        println!("BLPOP {}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[BRPOP]
            "brpop" => {
                if cmd_length < 3 {
                    println!("BRPOP {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }

                match usize::from_str_radix(usecmds[&usecmds.len() - 1].as_str(), 10) {
                    Ok(_) => unsafe {
                        self.brpop(vec);
                    },
                    Err(error) => {
                        println!("BRPOP {}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[BRPOPLPUSH]
            "brpoplpush" => {
                if cmd_length < 4 {
                    println!("BRPOPLPUSH {} 4", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    vec.push(&usecmds[i])
                }

                match usize::from_str_radix(usecmds[&usecmds.len() - 1].as_str(), 10) {
                    Ok(_) => unsafe {
                        self.brpoplpush(vec);
                    },
                    Err(error) => {
                        println!("BRPOPLPUSH {}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[LINDEX]
            "lindex" => {
                if cmd_length != 3 {
                    println!("LINDEX {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                match usize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(v) => unsafe { self.lindex(usecmds[1].as_str().to_string(), v) },
                    Err(error) => {
                        println!("LINDEX {}{}", constrs::CMD_IS_FAIL, error);
                        return;
                    }
                }
            }

            //[LINSERT]
            "linsert" => {
                if cmd_length != 5 {
                    println!("LINSERT {} 5", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                if usecmds[2].to_string() == "after" || usecmds[2].to_string() == "before" {
                    let mut vec: Vec<&str> = Vec::new();
                    for i in 1..usecmds.len() {
                        vec.push(&usecmds[i])
                    }
                    unsafe { self.linsert(vec) }
                } else {
                    println!("LINSERT before|after is wrong!{}", usecmds[2].to_string());
                    return;
                }
            }

            //[LLEN]
            "llen" => {
                if cmd_length != 2 {
                    println!("LLEN {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.llen(usecmds[1].to_string()) }
            }

            //[LPOP]
            "lpop" => {
                if cmd_length != 2 {
                    println!("LPOP {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.lpop(usecmds[1].to_string()) }
            }

            //[LPUSHX]
            "lpushx" => {
                if cmd_length < 3 {
                    println!("LPUSHX {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 2..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.lpushx(usecmds[1].to_string(), vec) }
            }

            //[LRANGE]
            "lrange" => {
                if cmd_length != 4 {
                    println!("LRANGE {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                match isize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(s) => match isize::from_str_radix(usecmds[3].as_str(), 10) {
                        Ok(e) => unsafe {
                            self.lrange(usecmds[1].to_string(), s as i32, e as i32);
                        },
                        Err(error) => println!("LRANGE end must number,{}", error),
                    },
                    Err(error) => println!("LRANGE start must number,{}", error),
                }
            }

            //[LREM]
            "lrem" => {
                if cmd_length != 4 {
                    println!("LREM {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                match isize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(c) => unsafe {
                        self.lrem(usecmds[1].to_string(), c, usecmds[3].to_string());
                    },
                    Err(error) => println!("LREM count must number,{}", error),
                }
            }

            //[LSET]
            "lset" => {
                if cmd_length != 4 {
                    println!("LSET {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }

                match isize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(c) => unsafe {
                        self.lset(usecmds[1].to_string(), c, usecmds[3].to_string());
                    },
                    Err(error) => println!("LSET index must number,{}", error),
                }
            }

            //[LTRIM]
            "ltrim" => {
                if cmd_length != 4 {
                    println!("LTRIM {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                match isize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(s) => match isize::from_str_radix(usecmds[3].as_str(), 10) {
                        Ok(e) => unsafe {
                            self.ltrim(usecmds[1].to_string(), s, e);
                        },
                        Err(error) => println!("LTRIM end must number,{}", error),
                    },
                    Err(error) => println!("LRANGE start must number,{}", error),
                }
            }

            //[RPOP]
            "rpop" => {
                if cmd_length != 2 {
                    println!("RPOP {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.rpop(usecmds[1].to_string()) }
            }

            //[RPOPLPUSH]
            "rpoplpush" => {
                if cmd_length != 3 {
                    println!("RPOPLPUSH {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                //构建args
                let mut args: Vec<&str> = Vec::new();
                for i in 1..usecmds.len() {
                    args.push(&usecmds[i])
                }
                unsafe { self.rpoplpush(args) }
            }

            //[RPUSH]
            "rpush" => {
                if cmd_length < 3 {
                    println!("RPUSH {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 2..usecmds.len() {
                    vec.push(&usecmds[i])
                }

                unsafe { self.rpush(usecmds[1].to_string(), vec) }
            }

            //[RPUSHX]
            "rpushx" => {
                if cmd_length < 3 {
                    println!("RPUSHX {} 4", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 2..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.rpushx(usecmds[1].to_string(), vec) }
            }

            _ => {
                println!("{}", constrs::CMD_IS_FAIL);
            }
        }
    }
}

pub trait RunUnsafe {
    //LIST
    unsafe fn rpushx(&self, key: String, values: Vec<&str>);
    unsafe fn rpush(&self, key: String, values: Vec<&str>);
    unsafe fn rpoplpush(&self, args: Vec<&str>);
    unsafe fn rpop(&self, key: String);
    unsafe fn ltrim(&self, key: String, start: isize, stop: isize);
    unsafe fn lset(&self, key: String, index: isize, value: String);
    unsafe fn lrem(&self, key: String, count: isize, value: String);
    unsafe fn lrange(&self, key: String, start: i32, stop: i32);
    unsafe fn lpushx(&self, key: String, values: Vec<&str>);
    unsafe fn lpop(&self, key: String);
    unsafe fn llen(&self, key: String);
    unsafe fn linsert(&self, args: Vec<&str>);
    unsafe fn lindex(&self, key: String, index: usize);
    unsafe fn brpoplpush(&self, args: Vec<&str>);
    unsafe fn brpop(&self, args: Vec<&str>);
    unsafe fn blpop(&self, args: Vec<&str>);
    unsafe fn lpush(&self, key: String, values: Vec<&str>);

    //HASH
    unsafe fn hscan(&self, keys: Vec<&str>);
    unsafe fn hvals(&self, key: String);
    unsafe fn hsetnx(&self, key: String, field: String, value: String);
    unsafe fn hmset(&self, keys: Vec<&str>);
    unsafe fn hmget(&self, keys: Vec<&str>);
    unsafe fn hlen(&self, key: String);
    unsafe fn hkeys(&self, key: String);
    unsafe fn hincrbyfloat(&self, key: String, field: String, value: String);
    unsafe fn hincrby(&self, key: String, field: String, incr_number: String);
    unsafe fn hgetall(&self, key: String);
    unsafe fn hexists(&self, key: String, field: String);
    unsafe fn hdel(&self, key: String, field: String);
    unsafe fn hget(&self, key: String, field: String);
    unsafe fn hset(&self, key: String, field: String, value: String);
    //STRING
    unsafe fn append(&self, key: String, value: String);
    unsafe fn decrby(&self, key: String, value: String);
    unsafe fn decr(&self, key: String);
    unsafe fn incrbyfloat(&self, key: String, value: String);
    unsafe fn incrby(&self, key: String, value: String);
    unsafe fn incr(&self, key: String);
    unsafe fn psetex(&self, key: String, timeout: usize, value: String);
    unsafe fn msetnx(&self, keys: Vec<&str>);
    unsafe fn mset(&self, keys: Vec<&str>);
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
    unsafe fn rpushx(&self, key: String, values: Vec<&str>) {
        for x in 0..values.len() {
            let c = &mut *self.clients; // redis client
            match c.lpushx(&key, &*values[x]) {
                Ok(_) => {
                    println!(
                        "{}) lpushx key {} value {} success!",
                        &x,
                        key.clone(),
                        &values[x]
                    )
                }
                Err(error) => {
                    println!(
                        "{}) lpushx key {} value {} failed! error {}",
                        &x,
                        key.clone(),
                        &values[x],
                        error.to_string()
                    )
                }
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn rpush(&self, key: String, values: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        for x in 0..values.len() {
            match c.lpush(&key, &*values[x]) {
                Ok(_) => {
                    println!(
                        "{}) rpush key {} value {} success!",
                        &x,
                        key.clone(),
                        &values[x]
                    )
                }
                Err(error) => {
                    println!(
                        "{}) rpush key {} value {} failed! error {}",
                        &x,
                        key.clone(),
                        &values[x],
                        error.to_string()
                    )
                }
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn rpoplpush(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<String>("RPOPLPUSH", args) {
            Ok(v) => {
                println!("RPOPLPUSH success value {}", v);
            }
            Err(error) => println!("RPOPLPUSH error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn rpop(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.rpop::<String>(&key) {
            Ok(v) => {
                let mut map_data = HashMap::new();
                map_data.insert(key, v);
                cvt_cmd::list::ListCMD {}.blpop(map_data)
            }
            Err(error) => println!("RPOP error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn ltrim(&self, key: String, start: isize, stop: isize) {
        let c = &mut *self.clients; // redis client
        match c.ltrim(&key, start, stop) {
            Ok(_) => {
                println!("LTRIM success");
            }
            Err(error) => println!("LTRIM wrong error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn lset(&self, key: String, index: isize, value: String) {
        let c = &mut *self.clients; // redis client
        match c.lset(&key, index, &*value) {
            Ok(_) => {
                println!("LSET success");
            }
            Err(error) => println!("LSET wrong error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn lrem(&self, key: String, count: isize, value: String) {
        let c = &mut *self.clients; // redis client
        match c.lrem(&key, count, &*value) {
            Ok(_) => {
                let mut header: Vec<String> = Vec::with_capacity(2);
                header.push("key".to_string());
                header.push("lrem_count".to_string());
                let mut data = HashMap::new();
                data.insert(key, count.abs().to_string());
                cvt_cmd::list::ListCMD {}.l_pub_k_v(header, data);
            }
            Err(error) => println!("LREM error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn lrange(&self, key: String, start: i32, stop: i32) {
        let c = &mut *self.clients; // redis client
        match c.lrange(
            &key,
            function::i32_2_isize(start),
            function::i32_2_isize(stop),
        ) {
            Ok(data) => {
                cvt_cmd::list::ListCMD {}.lrange(data);
            }
            Err(error) => println!("LRANGE error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn lpushx(&self, key: String, values: Vec<&str>) {
        for x in 0..values.len() {
            let c = &mut *self.clients; // redis client
            match c.lpushx(&key, &*values[x]) {
                Ok(_) => {
                    println!(
                        "{}) lpushx key {} value {} success!",
                        &x,
                        key.clone(),
                        &values[x]
                    )
                }
                Err(error) => {
                    println!(
                        "{}) lpushx key {} value {} failed! error {}",
                        &x,
                        key.clone(),
                        &values[x],
                        error.to_string()
                    )
                }
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn lpop(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.lpop::<String>(&key) {
            Ok(v) => {
                let mut map_data = HashMap::new();
                map_data.insert(key, v);
                cvt_cmd::list::ListCMD {}.blpop(map_data)
            }
            Err(error) => println!("LPOP error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn llen(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.llen(&key) {
            Ok(v) => cvt_cmd::list::ListCMD {}.llen(key, v),
            Err(error) => println!("LLEN error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn linsert(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("LINSERT", args.clone()) {
            Ok(v) => {
                println!("LINSERT success");
                cvt_cmd::list::ListCMD {}.llen(args[0].to_string(), v);
            }
            Err(error) => println!("LINSERT error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn lindex(&self, key: String, index: usize) {
        let c = &mut *self.clients; // redis client
        match c.lindex::<String>(&key, index.try_into().unwrap()) {
            Ok(value) => cvt_cmd::list::ListCMD {}.lindex(key, index, value),
            Err(error) => println!("LINDEX error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn brpoplpush(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<String>("BRPOPLPUSH", args.clone()) {
            Ok(v) => {
                let mut map_data = HashMap::new();
                map_data.insert(args[0].to_string(), v);
                cvt_cmd::list::ListCMD {}.blpop(map_data)
            }
            Err(error) => println!("BRPOPLPUSH error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn brpop(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<HashMap<String, String>>("BRPOP", args) {
            Ok(v) => cvt_cmd::list::ListCMD {}.blpop(v),
            Err(error) => println!("BRPOP error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn blpop(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<HashMap<String, String>>("BLPOP", args) {
            Ok(v) => cvt_cmd::list::ListCMD {}.blpop(v),
            Err(error) => {
                println!("BLPOP error {}", error)
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn lpush(&self, key: String, values: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        for x in 0..values.len() {
            match c.lpush(&key, &*values[x]) {
                Ok(_) => {
                    println!(
                        "{}) lpush key {} value {} success!",
                        &x,
                        key.clone(),
                        &values[x]
                    )
                }
                Err(error) => {
                    println!(
                        "{}) lpush key {} value {} failed! error {}",
                        &x,
                        key.clone(),
                        &values[x],
                        error.to_string()
                    )
                }
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn hscan(&self, keys: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("HSCAN", keys.clone()) {
            Ok(v) => {
                println!("hscan->{:?}", v);
            }
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn hvals(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.hvals(&key) {
            Ok(v) => {
                cvt_cmd::hash::HashCMD {}.hkeys(v);
            }
            Err(error) => println!("hvals hash key {} error : {}", key, error),
        }
    }

    #[allow(dead_code)]
    unsafe fn hsetnx(&self, key: String, field: String, value: String) {
        let c = &mut *self.clients; // redis client
        match c.hsetnx(&key, &field, &*value) {
            Ok(_) => {
                println!("hsetnx field {} success", field);
                self.hget(key, field)
            }
            Err(error) => println!("hsetnx hash key {} field {} error : {}", key, field, error),
        }
    }

    #[allow(dead_code)]
    unsafe fn hmset(&self, keys: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<String>("HMSET", keys.clone()) {
            Ok(v) => {
                println!("hmset {}", v)
            }
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn hmget(&self, keys: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("HMGET", keys.clone()) {
            Ok(v) => {
                let mut map_data = HashMap::new();
                for k in 1..keys.len() {
                    map_data.insert(keys[k].to_string(), v[k - 1].to_string());
                }
                cvt_cmd::string::StringCMD {}.mget(map_data);
            }
            Err(error) => println!("hmget hash key {}  error : {}", keys[0], error),
        }
    }

    #[allow(dead_code)]
    unsafe fn hlen(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("HLEN", vec![&key]) {
            Ok(v) => {
                println!("hlen hash key ({}) field length is {} !", key, v)
            }
            Err(error) => println!("hlen hash key {} error : {}", key, error),
        }
    }

    #[allow(dead_code)]
    unsafe fn hkeys(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.hkeys(&key) {
            Ok(v) => {
                cvt_cmd::hash::HashCMD {}.hkeys(v);
            }
            Err(error) => println!("hkeys key {} error : {}", key, error),
        }
    }

    #[allow(dead_code)]
    unsafe fn hincrbyfloat(&self, key: String, field: String, value: String) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<f64>("HINCRBYFLOAT", vec![&key, &field, &value]) {
            Ok(_) => {
                println!("hincrbyfloat key {} field {} success!", key, field);
                self.hget(key, field)
            }
            Err(error) => println!("hincrbyfloat key {} field {} error : {}", key, field, error),
        }
    }

    #[allow(dead_code)]
    unsafe fn hincrby(&self, key: String, field: String, incr_number: String) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("HINCRBY", vec![&key, &field, &incr_number]) {
            Ok(_) => {
                println!("hincrby key {} field {} success!", key, field);
                self.hget(key, field)
            }
            Err(error) => println!("hincrby key {} field {} error : {}", key, field, error),
        }
    }

    #[allow(dead_code)]
    unsafe fn hgetall(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.hgetall(&key) {
            Ok(map) => {
                cvt_cmd::hash::HashCMD {}.hgetall(map);
            }
            Err(error) => println!("Unable to read map from Redis: {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn hexists(&self, key: String, field: String) {
        let c = &mut *self.clients; // redis client
        match c.hexists(&key, &field) {
            Ok(v) => {
                if v {
                    println!(
                        "hexists hash key {} field {} exists!",
                        key.clone(),
                        field.clone()
                    );
                    match c.hget::<String>(&key, &field) {
                        Ok(v) => cvt_cmd::hash::HashCMD {}.hget(key, field, v),
                        Err(e) => {
                            println!("Hexists error: {}", e.to_string())
                        }
                    }
                } else {
                    println!(
                        "hexists hash key {} field {} not exists!",
                        key.clone(),
                        field.clone()
                    )
                }
            }
            Err(e) => {
                println!("hexists error {}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn hdel(&self, key: String, field: String) {
        let c = &mut *self.clients; // redis client
        match c.hdel(&key, &field) {
            Ok(_) => {
                println!("hdel hash key {} field {} success (^v^)", key, field);
            }
            Err(e) => {
                println!("Hdel error: {}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn hget(&self, key: String, field: String) {
        let c = &mut *self.clients; // redis client
        match c.hget::<String>(&key, &field) {
            Ok(v) => cvt_cmd::hash::HashCMD {}.hget(key, field, v),
            Err(e) => {
                println!("Hget error: {}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn hset(&self, key: String, field: String, value: String) {
        let c = &mut *self.clients; // redis client
        match c.hset(&key, &field, &*value) {
            Ok(_) => {
                println!("hset hash key {} field {} success (^v^)", key, field);
            }
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn append(&self, key: String, value: String) {
        let c = &mut *self.clients; // redis client
        match c.append(&key, &value) {
            Ok(_) => {
                println!("append {} success (^v^)", key.clone());
                self.get(key)
            }
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn decrby(&self, key: String, value: String) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i64>("DECRBY", vec![&key, &value]) {
            Ok(v) => {
                println!("(integer) {}", v)
            }
            Err(e) => {
                println!("decrby error:{}", e.to_string());
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn decr(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i64>("DECR", vec![&key]) {
            Ok(v) => {
                println!("(integer) {}", v)
            }
            Err(e) => {
                println!("incr error:{}", e.to_string());
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn incrbyfloat(&self, key: String, value: String) {
        let c = &mut *self.clients; // redis client
        match c.incrbyfloat(&key, &*value) {
            Ok(v) => {
                println!("(float64) {}", v)
            }
            Err(e) => {
                println!("incrbyfloat error:{}", e.to_string());
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn incrby(&self, key: String, value: String) {
        let c = &mut *self.clients; // redis client
        match c.incrby(&key, &*value) {
            Ok(v) => {
                println!("(integer) {}", v)
            }
            Err(e) => {
                println!("incrby error:{}", e.to_string());
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn incr(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.incr(&key) {
            Ok(v) => {
                println!("(integer) {}", v)
            }
            Err(e) => {
                println!("incr error:{}", e.to_string());
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn psetex(&self, key: String, timeout: usize, value: String) {
        let c = &mut *self.clients; // redis client
        match c.run_command_empty_response(
            "PSETEX",
            vec![&key, &*timeout.to_string(), &value.to_string()],
        ) {
            Err(error) => println!("Unable to psetex value in Redis: {}", error),
            _ => {
                println!("psetex success");
                self.get(key)
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn msetnx(&self, keys: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("MSETNX", keys.clone()) {
            Ok(v) => {
                println!("{}", v)
            }
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn mset(&self, keys: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<String>("MSET", keys.clone()) {
            Ok(v) => {
                println!("{}", v)
            }
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn strlen(&self, key: String) {
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
        match c.exists(&key) {
            Ok(result) => result,
            Err(_) => false,
        }
    }

    #[allow(dead_code)]
    unsafe fn ttl_key(&self, key: String) {
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
        let c = &mut *self.clients; // redis client
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
