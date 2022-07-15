use regex::Regex;

#[derive(Debug, Clone)]
pub struct TagRegs {}

use super::strexpres::{Express, StrExpress};
pub trait Regs {
    fn reg_match_quotation(&self, str: String) -> String;
    fn reg_replace_space(&self, old_str: String) -> String;
}

impl Regs for TagRegs {
    //turn quotation marks
    fn reg_match_quotation(&self, str: String) -> String {
        let stu = str.replace("\"", "\\u53cc\\u5f15\\u53f7");
        let reg = Regex::new(r#"\\u53cc\\u5f15\\u53f7(.)*\\u53cc\\u5f15\\u53f7"#).unwrap();
        match reg.find(&stu) {
            Some(v) => {
                let a = v.to_owned().as_str().to_string();
                let b = a.clone().replace(" ", "\\u7a7a\\u683c");
                let c = stu.replace(a.as_str(), &b);
                let d = c.replace("\\u53cc\\u5f15\\u53f7", "\"");
                d
            }
            None => str,
        }
    }

    fn reg_replace_space(&self, old_str: String) -> String {
        let rep_str =
            StrExpress {}.replace(&old_str, "\\u7a7a\\u683c".to_string(), " ".to_string());
        rep_str
    }
}
