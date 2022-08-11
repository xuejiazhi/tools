pub fn add_plus(mut x: i32) -> i32 {
    x = x + 1;
    x
}

// pub fn usize_2_string(v: usize) -> String {
//     let data = (v as i32).to_string();
//     data
// }

pub fn isize_2_string(v: isize) -> String {
    let data = (v as i32).to_string();
    data
}


pub fn i32_2_isize(v: i32) -> isize {
    v.try_into().unwrap()
}

pub fn capture_vec_string(data: Vec<String>, start: usize, length: usize) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let lens = if length > data.len().try_into().unwrap() {
        data.len() 
    } else {
        length as usize
    };
    if start as usize > lens {
        return vec;
    }
    for i in start..lens{
        vec.push(data[i].to_string())
    }
    vec
}
