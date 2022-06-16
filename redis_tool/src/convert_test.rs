#[test]
fn test_string_split(){
    let cmd = "get test";
    let s = cmd.split(" ");
    println!("=====>{:?}",s);

    println!("  
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
    (__/ \\__)")
}

#[test]
fn test_usize(){
    let v=usize::from_str_radix("xxxxx", 10).is_ok();
    println!("v->{}",&v)
}