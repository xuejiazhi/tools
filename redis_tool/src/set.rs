/**
 * 集合
 */
use crate::{cmd as cvt_cmd, cmd::set::Cmd, cvt};

pub trait Set {
    //SET
    unsafe fn sscan(&self, args: Vec<&str>);
    unsafe fn sunionstore(&self, args: Vec<&str>);
    unsafe fn sunion(&self, args: Vec<&str>);
    unsafe fn srem(&self, key: String, member: String);
    unsafe fn srandmember(&self, key: String, cnt: isize);
    unsafe fn spop(&self, key: String, cnt: isize);
    unsafe fn smove(&self, source_key: String, destination_key: String, member: String);
    unsafe fn smembers(&self, key: String);
    unsafe fn sismember(&self, key: String, member: String);
    unsafe fn sinterstore(&self, args: Vec<&str>);
    unsafe fn sinter(&self, args: Vec<&str>);
    unsafe fn sdiffstore(&self, args: Vec<&str>);
    unsafe fn sdiff(&self, keys: Vec<&str>);
    unsafe fn scard(&self, key: String);
    unsafe fn sadd(&self, key: String, member: String);
}

impl Set for cvt::Cvt {
    #[allow(dead_code)]
    unsafe fn sscan(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("SSCAN", args) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(2);
                header.push("number".to_string());
                header.push("union-member".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(2);
                    son_data.push(i.to_string());
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("SSCAN error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn sunionstore(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("SUNIONSTORE", args) {
            Ok(v) => {
                println!("sunionstore success integer ({})", v);
            }
            Err(error) => println!("SUNIONSTORE error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn sunion(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("SUNION", args) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("union-member".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("SUNION error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn srem(&self, key: String, member: String) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("SREM", vec![&key, &member]) {
            Ok(v) => {
                println!("srem success integer ({})", v);
            }
            Err(error) => println!("SREM error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn srandmember(&self, key: String, cnt: isize) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("SRANDMEMBER", vec![&key, &cnt.to_string()]) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("rand-member".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("SRANDMEMBER error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn spop(&self, key: String, cnt: isize) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("SPOP", vec![&key, &cnt.to_string()]) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("pop-value".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("SPOP error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn smove(&self, source_key: String, destination_key: String, member: String) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("SMOVE", vec![&source_key, &destination_key, &member]) {
            Ok(v) => {
                println!("smove success integer ({})", v);
            }
            Err(error) => println!("SMOVE error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn smembers(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.smembers(&key) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("value".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("SMEMBERS error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn sismember(&self, key: String, member: String) {
        let c = &mut *self.clients; // redis client
        match c.sismember(&key, &member) {
            Ok(v) => {
                if v {
                    println!("exists!")
                } else {
                    println!("none!")
                }
            }
            Err(error) => println!("SISMEMBER error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn sinterstore(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("SINTERSTORE", args.clone()) {
            Ok(v) => {
                println!("sinterstore success key {} ({})", args[0].to_string(), v)
            }
            Err(error) => println!("SINTERSTORE error {}", error),
        }
    }

    unsafe fn sinter(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("SINTER", args) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("sinter-value".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("SINTER error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn sdiffstore(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("SDIFFSTORE", args.clone()) {
            Ok(v) => {
                println!("sdiffstore success key {} ({})", args[0].to_string(), v)
            }
            Err(error) => println!("SDIFFSTORE error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn sdiff(&self, keys: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.sdiff(keys) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("diff-value".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("SDIFF error {}", error),
        }
    }

    #[allow(dead_code)]
    unsafe fn scard(&self, key: String) {
        let c = &mut *self.clients; // redis client
        match c.scard(&key) {
            Ok(c) => {
                let mut header: Vec<String> = Vec::with_capacity(3);
                header.push("key".to_string());
                header.push("type".to_string());
                header.push("value".to_string());
                let mut data = Vec::with_capacity(3);
                data.push(key.clone());
                data.push("set".to_string());
                data.push(c.abs().to_string());
                cvt_cmd::set::SetCMD {}.l_pub_k_v(header, data);
            }
            Err(_) => todo!(),
        }
    }

    #[allow(dead_code)]
    unsafe fn sadd(&self, key: String, member: String) {
        let c = &mut *self.clients; // redis client
        match c.sadd(&key, &member) {
            Ok(v) => {
                println!("sadd success ({})", v)
            }
            Err(error) => {
                println!(
                    "sadd key {} member {} failed! error {}",
                    key.clone(),
                    member,
                    error
                )
            }
        }
    }
}
