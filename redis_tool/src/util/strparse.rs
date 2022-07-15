use std::env;

// 摘自 https://www.cnblogs.com/perfei/p/15347863.html

#[derive(Debug, Clone)]
pub struct StrParse {
    args: Vec<String>,
}

impl StrParse {
    pub fn new() -> StrParse {
        let sc = StrParse {
            args: env::args().collect(),
        };
        sc
    }

    // pub fn to_i32(&self, mut data: &mut i32, flag: &str, default_value: &str, desc: &str) {
    //     let mut not_has = true;
    //     for (index, val) in self.args.iter().enumerate() {
    //         if index > 0 {
    //             let data_flag = flag;
    //             if val == data_flag {
    //                 not_has = false;
    //                 // println!("{:#?}:{:#?}",val,args[index+1]);
    //                 *data = (self.args[index + 1]).parse().expect("Not a number!");
    //                 break;
    //             }
    //         }
    //     }
    //     if not_has {
    //         *data = default_value.parse().expect("Not a number!");
    //     }
    // }

    // pub fn to_f64(&self, mut data: &mut f64, flag: &str, default_value: &str, desc: &str) {
    //     let mut not_has = true;
    //     for (index, val) in self.args.iter().enumerate() {
    //         if index > 0 {
    //             let data_flag = flag;
    //             if val == data_flag {
    //                 not_has = false;
    //                 // println!("{:#?}:{:#?}",val,args[index+1]);
    //                 *data = (self.args[index + 1]).parse().expect("Not a number!");
    //                 break;
    //             }
    //         }
    //     }
    //     if not_has {
    //         *data = default_value.parse().expect("Not a number!");
    //     }
    // }

    pub fn to_string(&self,  data: &mut String, flag: &str, default_value: &str) {
        let mut not_has = true;
        for (index, val) in self.args.iter().enumerate() {
            if index > 0 {
                let data_flag = flag;
                if val == data_flag {
                    not_has = false;
                    // println!("{:#?}:{:#?}",val,args[index+1]);
                    *data = String::from(&self.args[index + 1]);
                    break;
                }
            }
        }
        if not_has {
            *data = String::from(default_value);
        }
    }

    // pub fn to_bool(&self, mut data: &mut bool, flag: &str, default_value: &str, desc: &str) {
    //     let mut not_has = true;
    //     for (index, val) in self.args.iter().enumerate() {
    //         if index > 0 {
    //             let data_flag = flag;
    //             if val == data_flag {
    //                 not_has = false;
    //                 let mut v = false;
    //                 if default_value == "true" {
    //                     v = true;
    //                 }

    //                 *data = v;
    //                 break;
    //             }
    //         }
    //     }
    //     if not_has {
    //         *data = false;
    //     }
    // }
}
