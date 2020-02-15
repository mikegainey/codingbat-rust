#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
    let s = "michael";
    for chunk in s.as_bytes().chunks(2) {
        println!("{:?}", chunk);
    }
}
