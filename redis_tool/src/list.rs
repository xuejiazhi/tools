use std::collections::HashMap;

use crate::{cvt,cmd as cvt_cmd, cmd::list::Cmd, util::function};
pub trait List {
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
}

impl List for cvt::Cvt {
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
}