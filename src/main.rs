#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
}

// Array-2 > has22
// https://codingbat.com/prob/p121853

//     Given an array of ints, return true if the array contains a 2 next to a 2 somewhere.

//     has22([1, 2, 2]) → true
//     has22([1, 2, 1, 2]) → false
//     has22([2, 1, 2]) → false

fn has22(a: &[i32]) -> bool {
    let mut lastn = a[0];
    for x in 1 .. a.len() {
        if a[x] == 2 && lastn == 2 {
            return true
        }
        lastn = a[x];
    }
    return false
}

// Array-2 > more14
// https://codingbat.com/prob/p104627

//     Given an array of ints, return true if the number of 1's is greater than the number of 4's

//     more14([1, 4, 1]) → true
//     more14([1, 4, 1, 4]) → false
//     more14([1, 1]) → true
fn more14(a: &[u32]) -> bool {

    fn helper(a: &[u32], ones: u32, fours: u32) -> bool {
        if a.len() == 0 {
            return ones > fours
        } else {
            let head = &a[0];
            let tail = &a[1..];
            if *head == 1 {
                helper(tail, ones+1, fours)
            } else if *head == 4 {
                helper(tail, ones, fours+1)
            } else {
                helper(tail, ones, fours)
            }
        }
    }
    helper(&a, 0, 0) // trampoline with the input and accumulators initialized to zero
}

// #[test]
// fn has22_test() {
//     assert!(has22(&[1, 2, 2]) == true);
//     assert!(has22(&[1, 2, 1, 2]) == false);
//     assert!(has22(&[2, 1, 2]) == false);
// }

// #[test]
// fn more14_test() {
//     assert!(more14(&[1, 4, 1]) == true);
//     assert!(more14(&[1, 4, 1, 4]) == false);
//     assert!(more14(&[1, 1]) == true);
// }
