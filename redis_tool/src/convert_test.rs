#[test]
fn test_string_split(){
    let cmd = "get test";
    let s = cmd.split(" ");
    println!("=====>{:?}",s);
}