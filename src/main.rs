#![allow(dead_code)]

fn main() {
    println!("{}", front_times("michael", 3));
}

fn front_times(s: &str, n: usize) -> String {
    use std::cmp;

    let len = s.len();
    let lastx = cmp::min(3, len);
    let front = &s[..lastx];

    front.repeat(n)
}
