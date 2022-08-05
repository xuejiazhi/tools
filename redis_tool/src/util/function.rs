pub fn add_plus(mut x: i32) -> i32 {
    x = x + 1;
    x
}


pub fn usize_2_string(v:usize) -> String{
    let data = (v as i32).to_string();
    data
}

pub fn i32_2_isize(v:i32) -> isize{
    v.try_into().unwrap()
}