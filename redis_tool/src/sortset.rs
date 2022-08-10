/**
 * 有序集合
 */
use crate::cvt;

pub trait SortSet {
    //SORTSET
    unsafe fn zadd(&self, args: Vec<&str>);
}

impl SortSet for cvt::Cvt {
    unsafe fn zadd(&self, args: Vec<&str>) {
        let c = &mut *self.clients; // redis client
        match c.run_command::<i32>("ZADD", args) {
            Ok(v) => {
                println!("integer ({})", v);
            }
            Err(error) => println!("ZADD error {}", error),
        }
    }
}
