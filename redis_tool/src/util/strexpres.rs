#[derive(Debug, Clone)]
pub struct StrExpress {}

//定义接口
pub trait Express {
    fn replace(&self, old_str: &str, from: String, to: String) -> String;
    fn del_null(&self, cmdlist: Vec<&str>) -> Vec<String>;
}

impl Express for StrExpress {
    fn replace(&self, old_str: &str, from: String, to: String) -> String {
        let ret_str = old_str;
        ret_str.replace(&from, &to)
    }

    fn del_null(&self, cmdlist: Vec<&str>) -> Vec<String> {
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
}
