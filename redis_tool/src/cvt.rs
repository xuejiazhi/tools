use crate::{cmd as cvt_cmd, cvt_cmd::string::Cmd, util::strexpres::*};

/**
 * @explain order
 */
pub struct Cvt {
    cmd: String,
    clients: *mut simple_redis::client::Client,
}

impl Cvt {
    #[allow(dead_code)]
    pub fn convert(&self) {
        let items: Vec<&str> = self.cmd.split(" ").collect();
        let usecmds = StrExpress {}.del_null(items);
        // .del_null(items);

        let cmd_length = usecmds.len();
        if cmd_length == 0 {
            println!("{}", crate::constrs::constrs::CMD_IS_NIL);
            return;
        }

        //match cmd to oprate
        let cmdlist = String::from(&usecmds[0]).to_lowercase();
        match &cmdlist as &str {
            //get key
            "get" => {
                if cmd_length != 2 {
                    println!("GET {} 2", crate::constrs::constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }

                unsafe {
                    self.get(usecmds[1].to_string());
                }
            }

            //set key
            "set" => {
                if cmd_length != 3 {
                    println!("SET {} 3", crate::constrs::constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe {
                    self.set(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //del key
            "del" => {
                if cmd_length != 2 {
                    println!("DEL {} 2", crate::constrs::constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.del(usecmds[1].to_string()) }
            }

            //getset
            "getset" => {
                if cmd_length != 3 {
                    println!(
                        "GETSET {} 3",
                        crate::constrs::constrs::STRING_LENGTH_IS_FAIL
                    );
                    return;
                }

                unsafe {
                    self.get(usecmds[1].to_string());
                    self.set(usecmds[1].to_string(), usecmds[2].to_string());
                }
            }

            //keys
            "keys" => {
                if cmd_length != 2 {
                    println!("KEYS {} 2", crate::constrs::constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self.keys(usecmds[1].to_string()) }
            }

            //type
            "type" => {
                if cmd_length != 2 {
                    println!("TYPE {} 2", crate::constrs::constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
            }

            _ => {
                println!("{}", crate::constrs::constrs::CMD_IS_FAIL);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn keys(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.keys(&key) {
            Ok(strs) => {
                println!("datalist==>{:?}", strs);
                let mut _data_list:Vec<Vec<String>> = vec![];
            }
            Err(error) => {
                println!("get error: {}", error);
            }
        }
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

    #[allow(dead_code)]
    unsafe fn get_type(&self, key: String) -> String {
        //set ttl error
        let mut type_val: String = "none".to_string();
        let c: &mut simple_redis::client::Client = &mut *self.clients; // redis client
        match c.run_command::<String>("TYPE", vec![&key]) { 
            Ok(val) => {
                type_val = val.to_string();
            }
            Err(_error) => {
            }
        }
        type_val
    }
}
