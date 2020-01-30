// Array-2 > countEvens
// https://codingbat.com/prob/p162010

// Return the number of even ints in the given array.

// count_evens([2, 1, 2, 3, 4]) → 3
// count_evens([2, 2, 0]) → 3
// count_evens([1, 3, 5]) → 0

fn count_evens(array: &[i32]) -> i32 {
    array.iter()
        .filter(|&x| x % 2 == 0)
        .count() as i32
}

// Array-2 > sum13
// https://codingbat.com/prob/p127384

// Return the sum of the numbers in the array, returning 0 for an empty array.
// Except the number 13 is very unlucky, so it does not count and numbers that come immediately after a 13 also do not count.

// sum13([1, 2, 2, 1]) → 6
// sum13([1, 1]) → 2
// sum13([1, 2, 2, 1, 13]) → 6

fn sum13(array: &[i32]) -> i32 {
    let mut count = 0;
    let mut previous = None;
    for &a in array {
        if a == 13 {
            previous = Some(13);
            continue;
        } else if previous == Some(13) {
            previous = Some(a);
            continue;
        } else {
            count += a;
            previous = Some(a);
        }
    }
    count
}

// Array-2 > lucky13
// https://codingbat.com/prob/p194525

// Given an array of ints, return true if the array contains no 1's and no 3's.

// lucky13([0, 2, 4]) → true
// lucky13([1, 2, 3]) → false
// lucky13([1, 2, 4]) → false

fn lucky13(array: &[i32]) -> bool {
    array.iter()
        .filter(|&x| x == &1 || x == &3)
        .count() == 0
}

// Array-2 > fizzArray
// https://codingbat.com/prob/p180920

// Given a number n, create and return a new int array of length n, containing the numbers 0, 1, 2, ... n-1.
// The given n may be 0, in which case just return a length 0 array.

// fizz_array(4) → [0, 1, 2, 3]
// fizz_array(1) → [0]
// fizz_array(10) → [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

fn fizz_array1(n: u32) -> Vec<u32> {
    (0..n).collect::<Vec<u32>>()
}

fn fizz_array2(n: u32) -> Vec<u32> {
    let mut array = Vec::new();
    for x in 0..n {
        array.push(x);
    }
    array
}

// Array-2 > no14
// https://codingbat.com/prob/p136648

// Given an array of ints, return true if it contains no 1's or it contains no 4's.

// no14([1, 2, 3]) → true
// no14([1, 2, 3, 4]) → false
// no14([2, 3, 4]) → true

fn no14(array: &[i32]) -> bool {
    let has_one = array.contains(&1);
    let has_four = array.contains(&4);
    !(has_one && has_four)
}

// Array-2 > matchUp
// https://codingbat.com/prob/p136254

// Given arrays nums1 and nums2 of the same length,
// for every element in nums1, consider the corresponding element in nums2 (at the same index).
// Return the count of the number of times that the two elements differ by 2 or less, but are not equal.

// match_up([1, 2, 3], [2, 3, 10]) → 2
// match_up([1, 2, 3], [2, 3, 5]) → 3
// match_up([1, 2, 3], [2, 3, 3]) → 2

fn match_up(num1: &[i32], num2: &[i32]) -> u32 {
    num1.iter()
        .zip(num2.iter())
        .filter(|(&a, &b)| (a - b).abs() == 2 || (a - b).abs() == 1)
        .count() as u32
}

// Array-2 > modThree
// https://codingbat.com/prob/p159979

// Given an array of ints, return true if the array contains either 3 even or 3 odd values all next to each other.

// mod_three([2, 1, 3, 5]) → true
// mod_three([2, 1, 2, 5]) → false
// mod_three([2, 4, 2, 5]) → true

fn mod_three(array: &[i32]) -> bool {

    fn even(n: i32) -> bool { n % 2 == 0 }

    for x in 0 ..= array.len()-3 {
        // check for three consecutive even numbers
        if even(array[x]) && even(array[x+1]) && even(array[x+2]) {
            return true;
        }
        // check for three consecutive odd numbers
        if array[x..x+3].iter() // just to see if this would work, ... and it did!
            .all(|&x| x % 2 == 1) {
                return true;
            }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_evens_test() {
        assert_eq!(count_evens(&[2, 1, 2, 3, 4]), 3);
        assert_eq!(count_evens(&[2, 2, 0]), 3);
        assert_eq!(count_evens(&[1, 3, 5]), 0);
    }

    #[test]
    fn sum13_test() {
        assert_eq!(sum13(&[1, 2, 2, 1]), 6);
        assert_eq!(sum13(&[1, 1]), 2);
        assert_eq!(sum13(&[1, 2, 2, 1, 13]), 6);
        assert_eq!(sum13(&[1, 2, 2, 1, 13, 56]), 6);
        assert_eq!(sum13(&[13, 2, 3, 1, 3]), 7);
    }

    #[test]
    fn lucky13_test() {
        assert_eq!(lucky13(&[0, 2, 4]), true);
        assert_eq!(lucky13(&[1, 2, 3]), false);
        assert_eq!(lucky13(&[1, 2, 4]), false);
    }

    #[test]
    fn fizz_array1_test() {
        assert_eq!(fizz_array1(4), [0, 1, 2, 3]);
        assert_eq!(fizz_array1(1), [0]);
        assert_eq!(fizz_array1(10), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn fizz_array2_test() {
        assert_eq!(fizz_array2(4), [0, 1, 2, 3]);
        assert_eq!(fizz_array2(1), [0]);
        assert_eq!(fizz_array2(10), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn no14_test() {
        assert_eq!(no14(&[1, 2, 3]), true);
        assert_eq!(no14(&[1, 2, 3, 4]), false);
        assert_eq!(no14(&[2, 3, 4]), true);
    }

    #[test]
    fn match_up_test() {
        assert_eq!(match_up(&[1, 2, 3], &[2, 3, 10]), 2);
        assert_eq!(match_up(&[1, 2, 3], &[2, 3, 5]), 3);
        assert_eq!(match_up(&[1, 2, 3], &[2, 3, 3]), 2);
        assert_eq!(match_up(&[1, 2, 3, 4], &[5, 4, 3, 2]), 2);
    }

    #[test]
    fn mod_three_test() {
        assert_eq!(mod_three(&[2, 1, 3, 5]), true);
        assert_eq!(mod_three(&[2, 1, 2, 5]), false);
        assert_eq!(mod_three(&[2, 4, 2, 5]), true);
        assert_eq!(mod_three(&[2, 4, 7, 5, 1]), true);
    }
}
