#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
}

// make_last([4, 5, 6]) → [0, 0, 0, 0, 0, 6]
// make_last([1, 2]) → [0, 0, 0, 2]
// make_last([3]) → [0, 3]

fn make_last(a: &[i32]) -> &[i32] {
    let len = a.len();
    let mut output = Vec::new();
    output.push(a[len-1]);
    &output
}
