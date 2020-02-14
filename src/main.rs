#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
    let a: Option<u32> = Some(5);
    match a {
        Some(x) => println!("Some"),
        None => println!("None")
    }
}
