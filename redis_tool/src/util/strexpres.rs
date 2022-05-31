#[derive(Debug, Clone)]
pub struct StrExpress {}

//定义接口
pub trait Express {
    fn replace(&self, oldStr: &str, from: String, to: String) -> String;
}

impl Express for StrExpress {
    fn replace(&self, oldStr: &str, from: String, to: String) -> String {
        let mut retStr = oldStr;
        retStr.replace(&from, &to)
    }
}
