use crate::{
    cmd as cvt_cmd,
    cmd::string::Cmd,
    constrs::constrs,
    hash::Hash,
    help::route::Route,
    keys::Keys,
    list::List,
    set::Set,
    sortset::SortSet,
    stringopt::StringOpt,
    util::{
        function,
        strexpres::{Express, StrExpress},
        tagregs::{Regs, TagRegs},
    },
};

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
            // println!("{}", constrs::CMD_IS_NIL);
            return;
        }

        //match cmd to oprate
        let cmdlist = String::from(&usecmds[0]).to_lowercase();
        //命令
        let args = function::capture_vec_string(usecmds.clone(), 1, usecmds.len());
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
                        self.setrange(
                            usecmds[1].to_string(),
                            usecmds[2].to_string(),
                            usecmds[3].to_string(),
                        );
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

                match isize::from_str_radix(usecmds[2].as_str(), 10) {
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
                    println!("RPUSHX {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut vec: Vec<&str> = Vec::new();
                for i in 2..usecmds.len() {
                    vec.push(&usecmds[i])
                }
                unsafe { self.rpushx(usecmds[1].to_string(), vec) }
            }
            //Redis 集合的实现
            //Redis 的 Set 是 String 类型的无序集合。集合成员是唯一的，这就意味着集合中不能出现重复的数据。
            // 集合对象的编码可以是 intset 或者 hashtable。
            // Redis 中集合是通过哈希表实现的，所以添加，删除，查找的复杂度都是 O(1)。
            //集合中最大的成员数为 232 - 1 (4294967295, 每个集合可存储40多亿个成员)
            "sadd" => {
                if cmd_length != 3 {
                    println!("SADD {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.sadd(usecmds[1].to_string(), usecmds[2].to_string()) }
            }

            //[Scard]
            "scard" => {
                if cmd_length != 2 {
                    println!("SCARD {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.scard(usecmds[1].to_string()) }
            }

            //[SDIFF]
            "sdiff" => {
                if cmd_length < 3 {
                    println!("SDIFF {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let args = function::capture_vec_string(usecmds.clone(), 1, usecmds.len());
                unsafe { self.sdiff(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[SDIFFSTORE]
            "sdiffstore" => {
                if cmd_length != 4 {
                    println!("SDIFFSTORE {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                let args = function::capture_vec_string(usecmds.clone(), 1, usecmds.len());
                unsafe { self.sdiffstore(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[SINTER]
            "sinter" => {
                if cmd_length < 3 {
                    println!("SINTER {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let args = function::capture_vec_string(usecmds.clone(), 1, usecmds.len());
                unsafe { self.sinter(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[SINTERSTORE]
            "sinterstore" => {
                if cmd_length < 4 {
                    println!("SINTERSTORE {} 4", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let args = function::capture_vec_string(usecmds.clone(), 1, usecmds.len());
                unsafe { self.sinterstore(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[SISMEMBER]
            "sismember" => {
                if cmd_length != 3 {
                    println!("SISMEMBER {} 3", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.sismember(usecmds[1].to_string(), usecmds[2].to_string()) }
            }

            //[SMEMBERS]
            "smembers" => {
                if cmd_length != 2 {
                    println!("SMEMBERS {} 2", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.smembers(usecmds[1].to_string()) }
            }

            //[SMOVE]
            "smove" => {
                if cmd_length != 4 {
                    println!("SMOVE {} 4", constrs::CLI_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.smove(
                        usecmds[1].to_string(),
                        usecmds[2].to_string(),
                        usecmds[3].to_string(),
                    );
                }
            }

            //[SPOP]
            "spop" => {
                if cmd_length < 2 {
                    println!("SPOP {} 2", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                if cmd_length > 2 {
                    match isize::from_str_radix(usecmds[2].as_str(), 10) {
                        Ok(c) => unsafe { self.spop(usecmds[1].to_string(), c) },
                        Err(error) => println!("SPOP count must number,{}", error),
                    }
                } else {
                    unsafe { self.spop(usecmds[1].to_string(), 1) }
                }
            }

            //[SRANDMEMBER]
            "srandmember" => {
                if cmd_length < 2 {
                    println!("SRANDMEMBER {} 2", constrs::CLI_LENGTH_TAHN);
                    return;
                }

                let mut cnt: isize = 1;
                if cmd_length > 2 {
                    match isize::from_str_radix(usecmds[2].as_str(), 10) {
                        Ok(c) => {
                            cnt = c.abs();
                        }
                        Err(error) => println!("SRANDMEMBER count must number,{}", error),
                    }
                }
                unsafe { self.srandmember(usecmds[1].to_string(), cnt) }
            }

            //[SREM]
            "srem" => {
                if cmd_length < 3 {
                    println!("SREM {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                unsafe { self.srem(usecmds[1].to_string(), usecmds[2].to_string()) }
            }

            //[SUNION]
            "sunion" => {
                if cmd_length < 2 {
                    println!("SUNION {} 2", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let args = function::capture_vec_string(usecmds.clone(), 1, usecmds.len());
                unsafe { self.sunion(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[SUNIONSTORE]
            "sunionstore" => {
                if cmd_length < 3 {
                    println!("SUNIONSTORE {} 3", constrs::CLI_LENGTH_TAHN);
                    return;
                }
                let args = function::capture_vec_string(usecmds.clone(), 1, usecmds.len());
                unsafe { self.sunionstore(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[SSCAN]
            //有问题
            "sscan" => {
                if cmd_length != 3 && cmd_length != 5 && cmd_length != 7 {
                    println!("SSCAN ERR syntax error");
                    return;
                }

                match isize::from_str_radix(usecmds[2].as_str(), 10) {
                    Ok(_) => {
                        if cmd_length == 7 {
                            match isize::from_str_radix(usecmds[6].as_str(), 10) {
                                Ok(_) => {
                                    let args = function::capture_vec_string(
                                        usecmds.clone(),
                                        1,
                                        usecmds.len(),
                                    );
                                    unsafe {
                                        self.sscan(Vec::from_iter(args.iter().map(String::as_str)))
                                    }
                                }
                                Err(error) => println!("SSCAN count ERR syntax error:{}", error),
                            }
                        } else {
                            //7个CMD以下
                            let args =
                                function::capture_vec_string(usecmds.clone(), 1, usecmds.len());
                            unsafe { self.sscan(Vec::from_iter(args.iter().map(String::as_str))) }
                        }
                    }
                    Err(error) => println!("SSCAN cursor ERR syntax error:{}", error),
                }
            }

            //SORTSET 有序集合
            //[ZADD]
            "zadd" => {
                if !judgement_than(cmd_length, 4, "ZADD") {
                    return;
                };
                unsafe { self.zadd(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZCARD]
            "zcard" => {
                if !judgement_equal(cmd_length, 4, "ZCARD") {
                    return;
                };
                unsafe { self.zcard(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZCOUNT]
            "zcount" => {
                if !judgement_equal(cmd_length, 4, "ZCOUNT") {
                    return;
                };
                unsafe { self.zcount(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZINCRBY]
            "zincrby" => {
                if !judgement_equal(cmd_length, 4, "ZINCRBY") {
                    return;
                };
                unsafe { self.zincrby(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZINTERSTORE]
            "zinterstore" => {
                if !judgement_than(cmd_length, 4, "ZINTERSTORE") {
                    return;
                };
                unsafe { self.zinterstore(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZLEXCOUNT]
            "zlexcount" => {
                if !judgement_equal(cmd_length, 4, "ZLEXCOUNT") {
                    return;
                };
                unsafe { self.zlexcount(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZRANGE]
            "zrange" => {
                if !judgement_than(cmd_length, 4, "ZRANGE") {
                    return;
                };
                unsafe { self.zrange(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZRANGEBYLEX]
            "zrangebylex" => {
                if !judgement_than(cmd_length, 4, "ZRANGEBYLEX") {
                    return;
                };
                unsafe { self.zrangebylex(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZRANGEBYSCORE]
            "zrangebyscore" => {
                if !judgement_than(cmd_length, 4, "ZRANGEBYSCORE") {
                    return;
                };
                unsafe { self.zrangebyscore(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZRANK]
            "zrank" => {
                if !judgement_equal(cmd_length, 3, "ZRANK") {
                    return;
                };
                unsafe { self.zrank(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZREM]
            "zrem" => {
                if !judgement_than(cmd_length, 3, "ZREM") {
                    return;
                };
                unsafe { self.zrem(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZREMRANGEBYLEX]
            "zremrangebylex" => {
                if !judgement_equal(cmd_length, 4, "ZREMRANGEBYLEX") {
                    return;
                };
                unsafe { self.zremrangebylex(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZREMRANGEBYRANK]
            "zremrangebyrank" => {
                if !judgement_equal(cmd_length, 4, "ZREMRANGEBYRANK") {
                    return;
                };
                unsafe { self.zremrangebyrank(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZREMRANGEBYSCORE]
            "zremrangebyscore" => {
                if !judgement_equal(cmd_length, 4, "ZREMRANGEBYSCORE") {
                    return;
                };
                unsafe { self.zremrangebyscore(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZREVRANGE]
            "zrevrange" => {
                if !judgement_than(cmd_length, 4, "ZREVRANGE") {
                    return;
                };
                unsafe { self.zrevrange(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZSCORE]
            "zscore" => {
                if !judgement_equal(cmd_length, 3, "ZSCORE") {
                    return;
                };
                unsafe { self.zscore(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZUNIONSTORE]
            "zunionstore" => {
                if !judgement_than(cmd_length, 4, "ZUNIONSTORE") {
                    return;
                };
                unsafe { self.zunionstore(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            //[ZSCAN]
            "zscan" => {
                if !judgement_than(cmd_length, 3, "ZSCAN") {
                    return;
                };
                unsafe { self.zscan(Vec::from_iter(args.iter().map(String::as_str))) }
            }

            _ => {
                if usecmds.len() == 0 {
                    println!("")
                } else {
                    println!("{}", constrs::CMD_IS_FAIL);
                }
            }
        }

        //判断长度大于
        fn judgement_than(length: usize, min_length: usize, cmd_str: &str) -> bool {
            if length < min_length {
                println!("{} {} {}", cmd_str, constrs::CLI_LENGTH_TAHN, min_length);
                false
            } else {
                true
            }
        }

        //判断长度等于
        fn judgement_equal(length: usize, min_length: usize, cmd_str: &str) -> bool {
            if length != min_length {
                println!("{} {} {}", cmd_str, constrs::CLI_LENGTH_IS_FAIL, min_length);
                false
            } else {
                true
            }
        }
    }
}
