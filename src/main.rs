#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {

    // max_mirror(&[1, 2, 1, 4]);
    search(&[2,4,3,5,4,6,7,5,8,7,9], &[8,6,7,5]);
}

// max_mirror([1, 2, 3, 8, 9, 3, 2, 1]) → 3
// max_mirror([1, 2, 1, 4]) → 3
// max_mirror([7, 1, 2, 9, 7, 2, 1]) → 2
fn max_mirror(a: &[u32]) -> u32 {


    let alen = a.len();
    for length in (2..=alen).rev() {
        for leftx in 0..=alen-length {
            // println!("length = {}, leftx = {}", length, leftx);
        }
        println!("");
    }
    0
}

fn search(a: &[u32], sub: &[u32]) -> bool {
    let rsub: Vec<u32> = sub.iter().rev().cloned().collect();
    let (alen, rsublen) = (a.len(), rsub.len());
    for index in 0..=alen-rsublen {
        if a[index..index+rsublen].to_vec() == rsub {
            println!("found at index {}", index);
            return true;
        }
    }
    println!("not found");
    false
}
