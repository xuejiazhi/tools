use regex::Regex;
use std::{
    env,
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!(
            "########### Error : Must three params,like:
                  #> tcp_client.exe 192.168.1.88 8080 "
        );
    }

    let ip = &args[1];
    let cp = check_ip(String::from(ip));

    if !cp {
        panic!("######## Error: painc Ip is Failed")
    }

    let port = &args[2];

    if !check_isnumber(String::from(port)) {
        panic!("######## Error: painc port is failed")
    }

    let mut sendstr = "hello! welcome hytera!";
    if args.len() > 3 {
        sendstr = &args[3];
    }

    let mut url = String::new();
    url.push_str(ip);
    url.push(':');
    url.push_str(port);

    println!("Tcp Server -> {}", url);

    //connect tcp server
    let mut stream = TcpStream::connect(url).unwrap();

    //send String
    stream.write(sendstr.as_bytes()).unwrap();

    let mut count = 0;
    loop {
        println!("Receiving...");
        let mut buffer = [0; 520];
        stream.read(&mut buffer).unwrap();
        println!("request : {}", String::from_utf8_lossy(&buffer[..]));
        count = count + 1;
        if count > 10 {
            break;
        }
    }
    println!("######## End Tcp Test tools!")
}

//check ip is valid
fn check_ip(ipstr: String) -> bool {
    let r = Regex::new(
        r"^((\d|[1-9]\d|1\d\d|2[0-4]\d|25[0-5])\.){3}(\d|[1-9]\d|1\d\d|2[0-4]\d|25[0-5])$",
    )
    .unwrap();
    if !r.is_match(&ipstr) {
        false
    } else {
        true
    }
}

//check is number
fn check_isnumber(numstr: String) -> bool {
    for c in numstr.chars() {
        if !c.is_ascii_alphanumeric() {
            false;
        }
    }
    true
}
