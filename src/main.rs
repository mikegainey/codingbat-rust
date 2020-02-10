#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod string2;
use string2::plus_out;

fn main() {
    let a = string2::plus_out("krista mista", "st");
    println!("{}", a);
}


