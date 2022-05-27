#[derive(Debug)]
pub struct StringCMD {
    cmd: String,
    options: vec::new(),
}

pub mod cmd {
    pub mod hash;
    pub mod list; //list 操作
    pub mod string; //string 操作 //hash 操作
}