#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
    let array = vec![5,7,3];

    let largest = &array.iter().max().unwrap();

    println!("The largest number in {:?} is {}.", array, largest);
}


