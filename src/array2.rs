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

// Array-2 > sameEnds
// https://codingbat.com/prob/p134300

// Return true if the group of N numbers at the start and end of the array are the same.
// For example, with {5, 6, 45, 99, 13, 5, 6}, the ends are the same for n=0 and n=2, and false for n=1 and n=3.
// You may assume that n is in the range 0..nums.length inclusive.

// same_ends([5, 6, 45, 99, 13, 5, 6], 1) → false
// same_ends([5, 6, 45, 99, 13, 5, 6], 2) → true
// same_ends([5, 6, 45, 99, 13, 5, 6], 3) → false

fn same_ends(array: &[i32], n: usize) -> bool {
    let len = array.len();
    let firstn = &array[..n];      // let signals a new variable on the stack, which requires a
    let lastn = &array[(len-n)..]; // value of known size (a reference)
    firstn == lastn
}

// Array-2 > shiftLeft
// https://codingbat.com/prob/p105031

// Return an array that is "left shifted" by one -- so {6, 2, 5, 3} returns {2, 5, 3, 6}.
// You may modify and return the given array, or return a new array.

// shift_left([6, 2, 5, 3]) → [2, 5, 3, 6]
// shift_left([1, 2]) → [2, 1]
// shift_left([1]) → [1]

fn shift_left(array: &[i32]) -> Vec<i32> {
    let mut output = array[1..].to_vec();
    output.push(array[0]);
    output
}

// Array-2 > post4
// https://codingbat.com/prob/p164144

// Given a non-empty array of ints, return a new array
// containing the elements from the original array that come after the last 4 in the original array.
// The original array will contain at least one 4.

// post4([2, 4, 1, 2]) → [1, 2]
// post4([4, 1, 4, 2]) → [2]
// post4([4, 4, 1, 2, 3]) → [1, 2, 3]

fn post4(array: &[i32]) -> Vec<i32> {
    let len = array.len();
    for (x, a) in array.iter().rev().enumerate() {
        if a == &4 {
            return array[len-x..].to_vec();
        }
    }
    array.to_vec() // this should never happen because "the original array will contain at least one 4"
}

// Array-2 > withoutTen
// https://codingbat.com/prob/p196976

// Return a version of the given array where all the 10's have been removed.
// The remaining elements should shift left towards the start of the array as needed,
// and the empty spaces a the end of the array should be 0.
// So {1, 10, 10, 2} yields {1, 2, 0, 0}. You may modify and return the given array or make a new array.

// without_ten([1, 10, 10, 2]) → [1, 2, 0, 0]
// without_ten([10, 2, 10]) → [2, 0, 0]
// without_ten([1, 99, 10]) → [1, 99, 0]

fn without_ten1(array: &[i32]) -> Vec<i32> {
    let mut main_part: Vec<i32> = array.iter()
        .filter(|&x| x != &10)
        .cloned() // convert from &i32 to i32
        .collect();
    let zeros_needed = array.len() - main_part.len();
    let mut zeros = vec![0; zeros_needed];
    main_part.append(&mut zeros); // the mutability of the two vectors must match
    main_part
}

fn without_ten2(array: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    let mut ten_count = 0;
    for a in array.iter() {
        if a != &10 {
            output.push(*a);
        } else {
            ten_count += 1;
        }
    }
    let mut zeros = vec![0; ten_count];
    output.append(&mut zeros);
    output
}

// Array-2 > fizzBuzz
// https://codingbat.com/prob/p153059

// Consider the series of numbers beginning at start and running up to but not including end,
// so for example start=1 and end=5 gives the series 1, 2, 3, 4.
// Return a new String[] array containing the string form of these numbers,
// except for multiples of 3, use "Fizz" instead of the number,
// for multiples of 5 use "Buzz",
// and for multiples of both 3 and 5 use "FizzBuzz".

// fizz_buzz(1, 6) → ["1", "2", "Fizz", "4", "Buzz"]
// fizz_buzz(1, 8) → ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7"]
// fizz_buzz(1, 11) → ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz"]

fn fizz_buzz(start: usize, end: usize) -> Vec<String> {
    let mut output = Vec::new();
    for n in start..end {
        if n % 3 == 0 && n % 5 == 0 {
            output.push("FizzBuzz".to_string());
        } else if n % 3 == 0 {
            output.push("Fizz".to_string());
        } else if n % 5 == 0 {
            output.push("Buzz".to_string());
        } else {
            output.push(n.to_string());
        }
    }
    output
}

// Array-2 > bigDiff
// https://codingbat.com/prob/p196640

// Given an array length 1 or more of ints, return the difference between the largest and smallest values in the array.

// big_diff([10, 3, 5, 6]) → 7
// big_diff([7, 2, 10, 9]) → 8
// big_diff([2, 10, 7, 2]) → 8

fn big_diff(array: &[i32]) -> i32 {
    let mut largest = array[0];
    let mut smallest = array[0];
    for n in array.iter() {
        if n > &largest {
            largest = *n;
        } else if n < &smallest {
            smallest = *n;
        }
    }
    largest - smallest
}

// Array-2 > sum67
// https://codingbat.com/prob/p111327

// Return the sum of the numbers in the array,
// except ignore sections of numbers starting with a 6 and extending to the next 7 (every 6 will be followed by a 7).
// Return 0 for no numbers.

// sum67([1, 2, 2]) → 5
// sum67([1, 2, 2, 6, 99, 99, 7]) → 5
// sum67([1, 1, 6, 7, 2]) → 4

fn sum67(array: &[i32]) -> i32 {
    let mut sum = 0;
    let mut count = true;
    for a in array.iter() {
        if a == &6 {
            count = false;
        } else if a == &7 {
            count = true;
        } else if count {
            sum += a;
        }
    }
    sum
}

// Array-2 > sum28
// https://codingbat.com/prob/p199612

// Given an array of ints, return true if the sum of all the 2's in the array is exactly 8.

// sum28([2, 3, 2, 2, 4, 2]) → true
// sum28([2, 3, 2, 2, 4, 2, 2]) → false
// sum28([1, 2, 3, 4]) → false

fn sum28(array: &[i32]) -> bool {
    array.iter()
        .filter(|&x| x == &2)
        .sum::<i32>() == 8
}

// Array-2 > only14
// https://codingbat.com/prob/p186672

// Given an array of ints, return true if every element is a 1 or a 4.

// only14([1, 4, 1, 4]) → true
// only14([1, 4, 2, 4]) → false
// only14([1, 1]) → true

fn only14(array: &[i32]) -> bool {
    array.iter()
        .all(|&x| x == 1 || x == 4)
}

// Array-2 > isEverywhere
// https://codingbat.com/prob/p110222

// We'll say that a value is "everywhere" in an array if for every pair of adjacent elements in the array,
// at least one of the pair is that value. Return true if the given value is everywhere in the array.

// is_everywhere([1, 2, 1, 3], 1) → true
// is_everywhere([1, 2, 1, 3], 2) → false
// is_everywhere([1, 2, 1, 3, 4], 1) → false

fn is_everywhere(array: &[i32], val: i32) -> bool {
    let lastx = array.len() - 1;
    for x in 0..lastx {
        if !array[x..=x+1].contains(&val) {
            return false;
        }
    }
    true
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

    #[test]
    fn same_ends_test() {
        assert_eq!(same_ends(&[5, 6, 45, 99, 13, 5, 6], 1), false);
        assert_eq!(same_ends(&[5, 6, 45, 99, 13, 5, 6], 0), true);
        assert_eq!(same_ends(&[5, 6, 45, 99, 13, 5, 6], 2), true);
        assert_eq!(same_ends(&[5, 6, 45, 99, 13, 5, 6], 3), false);
    }

    #[test]
    fn shift_left_test() {
        assert_eq!(shift_left(&[6, 2, 5, 3]), [2, 5, 3, 6]);
        assert_eq!(shift_left(&[1, 2]), [2, 1]);
        assert_eq!(shift_left(&[1]), [1]);
    }

    #[test]
    fn post4_test() {
        assert_eq!(post4(&[2, 4, 1, 2]), [1, 2]);
        assert_eq!(post4(&[4, 1, 4, 2]), [2]);
        assert_eq!(post4(&[4, 4, 1, 2, 3]), [1, 2, 3]);
    }

    #[test]
    fn without_ten1_test() {
        assert_eq!(without_ten1(&[1, 10, 10, 2]), [1, 2, 0, 0]);
        assert_eq!(without_ten1(&[10, 2, 10]), [2, 0, 0]);
        assert_eq!(without_ten1(&[1, 99, 10]), [1, 99, 0]);
    }

    #[test]
    fn without_ten2_test() {
        assert_eq!(without_ten2(&[1, 10, 10, 2]), [1, 2, 0, 0]);
        assert_eq!(without_ten2(&[10, 2, 10]), [2, 0, 0]);
        assert_eq!(without_ten2(&[1, 99, 10]), [1, 99, 0]);
    }

    #[test]
    fn fizz_buzz_test() {
        assert_eq!(fizz_buzz(1, 6), ["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(fizz_buzz(1, 8), ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7"]);
        assert_eq!(fizz_buzz(1, 11), ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz"]);
    }

    #[test]
    fn big_diff_test() {
        assert_eq!(big_diff(&[10, 3, 5, 6]), 7);
        assert_eq!(big_diff(&[7, 2, 10, 9]), 8);
        assert_eq!(big_diff(&[2, 10, 7, 2]), 8);
    }

    #[test]
    fn sum67_test() {
        assert_eq!(sum67(&[1, 2, 2]), 5);
        assert_eq!(sum67(&[1, 2, 2, 6, 99, 99, 7]), 5);
        assert_eq!(sum67(&[1, 1, 6, 7, 2]), 4);
    }

    #[test]
    fn sum28_test() {
        assert_eq!(sum28(&[2, 3, 2, 2, 4, 2]), true);
        assert_eq!(sum28(&[2, 3, 2, 2, 4, 2, 2]), false);
        assert_eq!(sum28(&[1, 2, 3, 4]), false);
    }

    #[test]
    fn only14_test() {
        assert_eq!(only14(&[1, 4, 1, 4]), true);
        assert_eq!(only14(&[1, 4, 2, 4]), false);
        assert_eq!(only14(&[1, 1]), true);
    }

    #[test]
    fn is_everywhere_test() {
        assert_eq!(is_everywhere(&[1, 2, 1, 3], 1), true);
        assert_eq!(is_everywhere(&[1, 2, 1, 3], 2), false);
        assert_eq!(is_everywhere(&[1, 2, 1, 3, 4], 1), false);
    }
}
