/**
 * 有序集合
 */
use crate::{cmd as cvt_cmd, cmd::set::Cmd, cvt};

pub trait SortSet {
    //SORTSET
    unsafe fn zscan(&self, args: Vec<&str>);  //ok
    unsafe fn zunionstore(&self, args: Vec<&str>); //
    unsafe fn zscore(&self, args: Vec<&str>); //
    unsafe fn zrevrange(&self, args: Vec<&str>); //ok
    unsafe fn zremrangebyscore(&self, args: Vec<&str>); //ok
    unsafe fn zremrangebyrank(&self, args: Vec<&str>); //ok
    unsafe fn zremrangebylex(&self, args: Vec<&str>); //ok
    unsafe fn zrem(&self, args: Vec<&str>); //ok
    unsafe fn zrank(&self, args: Vec<&str>); //ok
    unsafe fn zrangebyscore(&self, args: Vec<&str>); //ok
    unsafe fn zrangebylex(&self, args: Vec<&str>); //ok
    unsafe fn zrange(&self, args: Vec<&str>);  //ok
    unsafe fn zlexcount(&self, args: Vec<&str>); //ok
    unsafe fn zinterstore(&self, args: Vec<&str>); //ok
    unsafe fn zincrby(&self, args: Vec<&str>); //ok
    unsafe fn zcount(&self, args: Vec<&str>); //ok
    unsafe fn zcard(&self, args: Vec<&str>); //ok
    unsafe fn zadd(&self, args: Vec<&str>);

    unsafe fn zrevrangebyscore(&self,args: Vec<&str>);
    unsafe fn zrevrank(&self,args: Vec<&str>);
}

impl SortSet for cvt::Cvt {
    unsafe fn zrevrangebyscore(&self,args: Vec<&str>){
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("ZREVRANGEBYSCORE", args) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("zrevrangebyscore-member".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("ZREVRANGEBYSCORE error {}", error),
        }
    }

    unsafe fn zrevrank(&self,args: Vec<&str>){
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZREVRANK", args) {
            Ok(v) => {
                println!("ZREVRANK integer ({})", v);
            }
            Err(error) => println!("ZREVRANK error {}", error),
        }
    }

    unsafe fn zscan(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<Vec<String>>>("ZSCAN", args) {
            Ok(v) => {
                if v.len() > 0 {
                    let mut header: Vec<String> = Vec::with_capacity(2);
                    header.push("number".to_string());
                    header.push("zscan-value".to_string());
                    let mut data: Vec<Vec<String>> = Vec::new();
                    for i in 0..v[0].len() {
                        let mut son_data = Vec::with_capacity(2);
                        son_data.push(i.to_string());
                        son_data.push(v[0][i].to_string());
                        data.push(son_data);
                    }
                    cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
                } else {
                    println!("ZSCAN value is none!");
                }
            }
            Err(error) => println!("ZSCAN error {}", error),
        }
    }

    unsafe fn zunionstore(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZUNIONSTORE", args) {
            Ok(v) => {
                println!("ZUNIONSTORE integer ({})", v);
            }
            Err(error) => println!("ZUNIONSTORE error {}", error),
        }
    }

    unsafe fn zscore(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<String>("ZSCORE", args) {
            Ok(v) => {
                println!("ZSCORE ({})", v);
            }
            Err(error) => println!("ZSCORE error {}", error),
        }
    }

    unsafe fn zrevrange(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("ZREVRANGE", args) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("zrevrange-member".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("ZREVRANGE error {}", error),
        }
    }

    unsafe fn zremrangebyscore(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZREMRANGEBYSCORE", args) {
            Ok(v) => {
                println!("ZREMRANGEBYSCORE integer ({})", v);
            }
            Err(error) => println!("ZREMRANGEBYSCORE error {}", error),
        }
    }

    unsafe fn zremrangebyrank(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZREMRANGEBYRANK", args) {
            Ok(v) => {
                println!("ZREMRANGEBYRANK integer ({})", v);
            }
            Err(error) => println!("ZREMRANGEBYRANK error {}", error),
        }
    }

    unsafe fn zremrangebylex(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZREMRANGEBYLEX", args) {
            Ok(v) => {
                println!("ZREMRANGEBYLEX integer ({})", v);
            }
            Err(error) => println!("ZREMRANGEBYLEX error {}", error),
        }
    }

    unsafe fn zrem(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZREM", args) {
            Ok(v) => {
                println!("ZREM integer ({})", v);
            }
            Err(error) => println!("ZREM error {}", error),
        }
    }

    unsafe fn zrank(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZRANK", args) {
            Ok(v) => {
                println!("ZRANK integer ({})", v);
            }
            Err(error) => println!("ZRANK error {}", error),
        }
    }

    unsafe fn zrangebyscore(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("ZRANGEBYSCORE", args) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("rangebyscore-member".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("ZRANGEBYSCORE error {}", error),
        }
    }

    unsafe fn zrangebylex(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("ZRANGEBYLEX", args) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("rangebylex-member".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("ZRANGEBYLEX error {}", error),
        }
    }

    unsafe fn zrange(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<Vec<String>>("ZRANGE", args) {
            Ok(v) => {
                let mut header: Vec<String> = Vec::with_capacity(1);
                header.push("range-member".to_string());
                let mut data: Vec<Vec<String>> = Vec::new();
                for i in 0..v.len() {
                    let mut son_data = Vec::with_capacity(1);
                    son_data.push(v[i].to_string());
                    data.push(son_data);
                }
                cvt_cmd::set::SetCMD {}.l_pub_k_mv(header, data);
            }
            Err(error) => println!("ZRANGE error {}", error),
        }
    }

    unsafe fn zlexcount(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZLEXCOUNT", args) {
            Ok(v) => {
                println!("ZLEXCOUNT integer ({})", v);
            }
            Err(error) => println!("ZLEXCOUNT error {}", error),
        }
    }

    unsafe fn zinterstore(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZINTERSTORE", args) {
            Ok(v) => {
                println!("ZINTERSTORE integer ({})", v);
            }
            Err(error) => println!("ZINTERSTORE error {}", error),
        }
    }

    unsafe fn zincrby(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<String>("ZINCRBY", args) {
            Ok(v) => {
                println!("ZINCRBY ({})", v);
            }
            Err(error) => println!("ZINCRBY error {}", error),
        }
    }

    unsafe fn zcount(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZCOUNT", args) {
            Ok(v) => {
                println!("ZCOUNT integer ({})", v);
            }
            Err(error) => println!("ZCOUNT error {}", error),
        }
    }

    unsafe fn zcard(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZCARD", args) {
            Ok(v) => {
                println!("ZCARD integer ({})", v);
            }
            Err(error) => println!("ZCARD error {}", error),
        }
    }

    unsafe fn zadd(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZADD", args) {
            Ok(v) => {
                println!("ZADD integer ({})", v);
            }
            Err(error) => println!("ZADD error {}", error),
        }
    }
}
