use std::collections::HashMap;

use crate::{cmd as cvt_cmd, cmd::string::Cmd, cvt, keys::Keys};
pub trait StringOpt {
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
}

impl StringOpt for cvt::Cvt {
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

}
