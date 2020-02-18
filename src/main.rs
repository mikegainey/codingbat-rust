#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
    let a = [1, 2, 2, 2, 5, 2];
    let start = 1;
    let cl = clump_length(start, &a);
    println!("clump length of {:?} starting at {} is {}", &a, start, cl);
}

// returns the clump length of the value at index start
fn clump_length(start: usize, array: &[i32]) -> usize {
    let mut clump_length = 0;
    for index in start..array.len() {
        if array[index] == array[start] {
            clump_length += 1;
        } else {
            break;
        }
    }
    clump_length
}
