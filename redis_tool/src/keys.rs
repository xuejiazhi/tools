use std::collections::HashMap;

use crate::{
    cmd as cvt_cmd,
    cmd::{hash::Cmd, string::Cmd as StringCMD},
    cvt, stringopt::StringOpt,
};

pub trait Keys {
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

impl Keys for cvt::Cvt {
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