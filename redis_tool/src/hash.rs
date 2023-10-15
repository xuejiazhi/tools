use crate::{
    cmd as cvt_cmd,
    cmd::{hash::Cmd, set::Cmd as SetCMD, string::Cmd as StringCMD},
    cvt,
};
use std::collections::HashMap;

pub trait Hash {
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
}

impl Hash for cvt::Cvt {
    #[allow(dead_code)]
    unsafe fn hscan(&self, keys: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<Vec<String>>>("HSCAN", keys.clone()) {
            Ok(v) => {
                if v.len() > 0 {
                    if v.len() > 0 {
                        let mut header: Vec<String> = Vec::with_capacity(2);
                        header.push("number".to_string());
                        header.push("hscan-value".to_string());
                        let mut data: Vec<Vec<String>> = Vec::new();
                        for i in 0..v[0].len() {
                            let mut son_data = Vec::with_capacity(2);
                            son_data.push(i.to_string());
                            son_data.push(v[0][i].to_string());
                            data.push(son_data);
                        }
                        cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
                    } else {
                        println!("HSCAN value is none!");
                    }
                }
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
}
