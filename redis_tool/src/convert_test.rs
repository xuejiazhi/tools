#[test]
fn test_string_split() {
    let cmd = "get test";
    let s = cmd.split(" ");
    println!("=====>{:?}", s);

    println!(
        "  
    .-\"\"\"-.
    / .===. \\
    \\/ 6 6 \\/
    ( \\___/ )
 _________ooo__\\_____/_____________
/                                  \\
|    Connect Redis Success!         |
\\_______________________ooo________/ 
     |  |  |
     |_ | _|
     |  |  |
     |__|__|
     /-'Y'-\\
    (__/ \\__)"
    )
}

#[test]
fn test_usize() {
    let v = usize::from_str_radix("xxxxx", 10).is_ok();
    println!("v->{}", &v)
}

#[test]
fn test_regex() {
    let t = "set xjz \"hello world\"";
    let st = t.replace("\"", "\\u53cc\\u5f15\\u53f7");
    println!("{}", st.clone());
    // let r = Regex::new(r#"((\$quote\\;){1}(.)*(\$quote\\;){1}\b)"#).unwrap();

    let r = regex::Regex::new(r#"\\u53cc\\u5f15\\u53f7(.)*&patch;"#).unwrap();
    // if r.is_match(&st) { // if let m = r.find(s) {
    //     println!("Found Matches:")
    // }

    let m = r.find(&st).unwrap().as_str();
    println!("match -> {}", m.clone());
    let ms = m.replace(" ", "&space;");
    println!("MS:=>{}", ms)

    //     let ms =base64::encode(m);
    //     println!("ms=>{}",ms);

    //     // let msc = base64::decode(ms.clone()).unwrap();
    //     // println!("{:?}",msc);

    //    let _bytes = base64::decode(ms).unwrap().as_slice();
}

#[test]
fn test_capture() {
    let str = "mset x 'xjz   cjb'  y 'hr 111'";
    let bytes = str.as_bytes();
    let mut p = 0;
    let mut cap = "".to_string();
    let mut cv = false;
    for &item in bytes.iter() {
        let chr = item as char;
        //判断是否是单引号
        if chr.to_string() == "'" {
            p = add_test(p);
            if p.clone() % 2 == 0 {
                cv = false;
            } else {
                cv = true;
            }
            println!("cv->{}", cv);
        }

        if cv && &chr.to_string() == " " {
            cap += &"\\u7a7a\\u683c".to_string()
        } else {
            cap += &chr.to_string();
        }

        println!("txt->{}", cap);
    }
    fn add_test(mut x: i32) -> i32 {
        x = x + 1;
        x
    }
}

#[test]
fn test_str2f64() {
    // let s="12esss3.456";
    // match s.parse::<f64>() {
    //     Ok(v) => {
    //         println!("ok")
    //     },
    //     Err(e) => {
    //         println!("error=>{}",e.to_string())
    //     },
    // }
    let mut s = -2;
}

#[test]
fn test_capture_vec() {
    let mut a1:Vec<String> = Vec::with_capacity(5);
    a1.push("1".to_string());
    a1.push("2".to_string());
    a1.push("3".to_string());
    a1.push("4".to_string());
    a1.push("5".to_string());
    println!("v->{:?}",&a1);
    let v = Vec::from_iter(a1.iter().map(String::as_str));

}
