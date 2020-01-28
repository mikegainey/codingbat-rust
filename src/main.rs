#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

// Given two ints, each in the range 10..99,
// return true if there is a digit that appears in both numbers, such as the 2 in 12 and 23.
// (Note: division, e.g. n/10, gives the left digit while the % "mod" n%10 gives the right digit.)

// share_digit(12, 23) → true
// share_digit(12, 43) → false
// share_digit(12, 44) → false

fn main() {
    println!("share_digit2(12, 23) -> {}", share_digit2(12, 23));
    println!("share_digit2(12, 43) -> {}", share_digit2(12, 43));
    println!("share_digit2(12, 44) -> {}", share_digit2(12, 44));
}


fn share_digit2(a: i32, b: i32) -> bool {
    let mut va = Vec::new();
    let mut vb = Vec::new();
    va.push(a / 10);
    va.push(a % 10);
    vb.push(b / 10);
    vb.push(b % 10);
    let mut match_found = false;
    for x in &va {
        for y in &vb {
            if x == y {
                match_found = true;
            }
        }
    }
    match_found
}
