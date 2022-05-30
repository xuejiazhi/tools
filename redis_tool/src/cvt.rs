use crate::cmd as cvt_cmd;
extern crate prettytable;

#[macro_use]
use prettytable::{Table, row, Row, Cell, cell};

/**
 * @explain order
 */
pub struct Cvt {
    cmd: String,
    clients: *mut simple_redis::client::Client,
}

impl Cvt {
    pub fn convert(&self) {
        // pub fn convert(&self) {
        let items: Vec<&str> = self.cmd.split(" ").collect();
        let usecmds = self::Cvt::del_null(items);

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
                    println!("GET {} 2", crate::constrs::Constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }

                unsafe {
                    self::Cvt::get(&self, usecmds[1].to_string());
                }
            }
            //set key
            "set" => {
                if cmd_length != 3 {
                    println!("SET {} 3", crate::constrs::Constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self::Cvt::set(&self, usecmds[1].to_string(), usecmds[2].to_string()) }
            }
            //del key
            "del" => {
                if cmd_length != 2 {
                    println!("DEL {} 2", crate::constrs::Constrs::STRING_LENGTH_IS_FAIL);
                    return;
                }
                unsafe { self::Cvt::del(&self, usecmds[1].to_string()) }
            }

            _ => {
                println!("{}", crate::constrs::Constrs::CMD_IS_FAIL);
            }
        }
    }

    unsafe fn get(&self, key: String) {
        let (ttl, err) = self::Cvt::get_ttl(&self, key.to_string());
        let c: &mut simple_redis::client::Client = &mut *self.clients; // first layer
        let str_val = c.get::<String>(&key);
        match str_val {
            Ok(strs) => {
                cvt_cmd::string::StringCMD {
                    key: key.to_string(),
                    val: strs.to_string(),
                    ttl: ttl,
                    err: err,
                }
                .get();
            }
            Err(error) => {
                println!("get error: {}", error);
            }
        }
    }

    unsafe fn set(&self, key: String, value: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // first layer
        match c.set(key.as_str(), value.as_str()) {
            Ok(_) => {
                println!(
                    "{} Key =>{},Val=>{}",
                    crate::constrs::Constrs::STRING_SET_REDIS_SUCCESS,
                    key,
                    value
                );
            }
            Err(error) => {
                println!(
                    "{} Key =>{},Val=>{}",
                    crate::constrs::Constrs::STRING_SET_REDIS_FAIL,
                    key,
                    value
                );
            }
        }
    }

    unsafe fn del(&self, key: String) {
        let c: &mut simple_redis::client::Client = &mut *self.clients; // first layer
        match c.del(key.as_str()) {
            Ok(_) => {
                println!(
                    "Key =>{},{} {}",
                    key,
                    crate::constrs::Constrs::DEL_REDIS_KEY,
                    " Success!"
                );
            }
            Err(error) => {
                println!(
                    "Key =>{},{} {},{}",
                    key,
                    crate::constrs::Constrs::DEL_REDIS_KEY,
                    " Fail!",
                    error.to_string(),
                );
            }
        }
    }

    fn del_null(cmdlist: Vec<&str>) -> Vec<String> {
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

    unsafe fn get_ttl(&self, key: String) -> (String, String) {
        //set ttl error
        let mut ttl_val: String = "nil".to_string();
        let mut err_val: String = "nil".to_string();
        let c: &mut simple_redis::client::Client = &mut *self.clients; // first layer
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
