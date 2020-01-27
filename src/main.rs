#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
    let s = "mike";
    let bs = s.as_bytes();
    println!("{:?}", bs);
    println!("{}", s);
    println!("{}", bs[0]);
    println!("{}", bs[0] == b'm');
}

