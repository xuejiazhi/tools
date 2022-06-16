use crate::{
    cmd as cvt_cmd,
    cmd::string::Cmd,
    constrs::constrs,
    util::strexpres::{Express, StrExpress},
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
        let items: Vec<&str> = self.cmd.split(" ").collect();
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
            // @ Redis Key Operate
            // like <del,dump,keys,type,exists,pttl,ttl>
            // Keys used to manage redis
            // [del]  This command is used to delete a key when it exists.
            "del" => {
                if cmd_length != 2 {
                    println!("DEL {} 2", constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.del(usecmds[1].to_string()) }
            }

            //[dump] Serialize the given key and return the serialized value.
            "dump" => {
                if cmd_length != 2 {
                    println!("DUMP {} 2", constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.dump(usecmds[1].to_string());
                }
            }

            //[keys] Find all keys that match the given pattern.
            "keys" => {
                if cmd_length != 2 {
                    println!("KEYS {} 2", constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.keys(usecmds[1].to_string()) }
            }

            //[type] Returns the type of the value stored by the key.
            "type" => {
                if cmd_length != 2 {
                    println!("TYPE {} 2", constrs::STRING_LENGTH_IS_FAIL);
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
                    println!("EXISTS {} 2", constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { println!("{}", self.key_is_exists(usecmds[1].to_string())) }
            }

            //[pttl]
            //Returns the remaining expiration time of the key in milliseconds
            "pttl" => {
                if cmd_length != 2 {
                    println!("PTTL {} 2", constrs::STRING_LENGTH_IS_FAIL);
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
                    println!("TTL {} 2", constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.ttl_key(usecmds[1].to_string());
                }
            }

            //[expire]
            "expire" => {
                if cmd_length != 3 {
                    println!("EXPIRE {} 3", constrs::STRING_LENGTH_IS_FAIL);
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
                    println!("PEXPIRE {} 3", constrs::STRING_LENGTH_IS_FAIL);
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
                    println!("Expireat {} 3", constrs::STRING_LENGTH_IS_FAIL);
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

            // @ Redis String Operate
            // like <del,dump,keys,type,exists,pttl>
            // Commands related to redis string data types are used to manage redis string values
            // [get] The get command is used to get the value of the specified key.
            // If the key does not exist, nil is returned.
            // If the value stored by key is not of string type, an error is returned
            "get" => {
                if cmd_length != 2 {
                    println!("GET {} 2", constrs::STRING_LENGTH_IS_FAIL);
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
                    println!("SET {} 3", constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.set(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //getset
            "getset" => {
                if cmd_length != 3 {
                    println!("GETSET {} 3", constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }

                unsafe {
                    self.get(usecmds[1].to_string());
                    self.set(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            _ => {
                println!("{}", constrs::CMD_IS_FAIL);
            }
        }
    }
}

pub trait RunUnsafe {
    unsafe fn expireat(&self, key: String, timestamp: String);
    unsafe fn pexpire(&self, key: String, millseconds: usize);
    unsafe fn expire(&self, key: String, seconds: usize);
    unsafe fn dump(&self, key: String);
    unsafe fn keys(&self, key: String);
    unsafe fn get(&self, key: String);
    unsafe fn set(&self, key: String, value: String);
    unsafe fn del(&self, key: String);
    unsafe fn get_type(&self, key: String) -> String;
    unsafe fn pttl_key(&self, key: String);
    unsafe fn ttl_key(&self, key: String);
    unsafe fn key_is_exists(&self, key: String) -> bool;
    unsafe fn get_ttl(&self, key: String) -> (String, String);
}

impl RunUnsafe for Cvt {
    #[allow(dead_code)]
    unsafe fn expireat(&self, key: String, timestamp: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<u64>("EXPIREAT", vec![&timestamp]) {
            Ok(_) => {
                println!("expireat success");
                self.get(key);
            }
            Err(_) => {
                println!("expireat fail!")
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn pexpire(&self, key: String, millseconds: usize) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.pexpire(&key, millseconds) {
            Ok(_) => {
                println!("expire success");
                self.get(key);
            }
            Err(_) => {
                println!("expire fail!")
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn expire(&self, key: String, seconds: usize) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.expire(&key, seconds) {
            Ok(_) => {
                println!("expire success");
                self.get(key);
            }
            Err(_) => {
                println!("expire fail!")
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
