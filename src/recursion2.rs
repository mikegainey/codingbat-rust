// Recursion-2 > groupSum

// Given an array of ints,
// is it possible to choose a group of some of the ints, such that the group sums to the given target?
// This is a classic backtracking recursion problem.
// Once you understand the recursive backtracking strategy in this problem,
// you can use the same pattern for many problems to search a space of choices.
// Rather than looking at the whole array,
// our convention is to consider the part of the array starting at index start and continuing to the end of the array.
// The caller can specify the whole array simply by passing start as 0.
// No loops are needed -- the recursive calls progress down the array.

// group_sum(0, [2, 4, 8], 10) → true
// group_sum(0, [2, 4, 8], 14) → true
// group_sum(0, [2, 4, 8], 9) → false

fn group_sum(start: usize, array: &[i32], target: i32) -> bool {
    if target == 0 {
        true
    } else if start == array.len()-1 {
        array[start] == target
    } else {
        group_sum(start+1, array, target - array[start]) ||
            group_sum(start+1, array, target)
    }
}

// Recursion-2 > groupSum5
// https://codingbat.com/prob/p138907

// Given an array of ints,
// is it possible to choose a group of some of the ints, such that the group sums to the given target
// with these additional constraints: all multiples of 5 in the array must be included in the group.
// If the value immediately following a multiple of 5 is 1, it must not be chosen. (No loops needed.)

// group_sum5(0, [2, 5, 10, 4], 19) → true
// group_sum5(0, [2, 5, 10, 4], 17) → true
// group_sum5(0, [2, 5, 10, 4], 12) → false
// group_sum5(0, [2, 5, 10, 1, 4], 16) → false
// group_sum5(0, [2, 6, 10, 1, 4], 16) → true

// base: if start is array.len(), then true iff target == 0
// if the number is a multiple of 5
// -> gs(start+1, array, target-array[start]) // take the number
// else if the number is 1 and the previous number was a multiple of 5
// -> gs(start+1, array, target) // don't take the number
// else the number must not be multiple of 5,
// -> gs(start+1, array, target-array[start]) || gs(start+1, array, target) // either take it or not

fn group_sum5(start: usize, array: &[i32], target: i32) -> bool {
    println!("start = {}, array = {:?}, target = {}", start, array, target);
    if start == array.len() {
        target == 0
    } else {
        if array[start] % 5 == 0 {
            group_sum5(start+1, array, target-array[start])
        } else if start > 0 && array[start] == 1 && array[start-1] % 5 == 0 {
            group_sum5(start+1, array, target)
        } else {
            group_sum5(start+1, array, target-array[start]) || group_sum5(start+1, array, target)
        }
    }
}

// Recursion-2 > splitOdd10
// https://codingbat.com/prob/p171660

// Given an array of ints, is it possible to divide the ints into two groups,
// so that the sum of one group is a multiple of 10, and the sum of the other group is odd.
// Every int must be in one group or the other.
// Write a recursive helper method that takes whatever arguments you like,
// and make the initial call to your recursive helper from splitOdd10(). (No loops needed.)

// split_odd10([5, 5, 5]) → true
// split_odd10([5, 5, 6]) → false
// split_odd10([5, 5, 6, 1]) → true

// fn split_odd10(array: &[i32]) -> bool {

//     fn mult_of_10(array: &[i32]) -> bool {
//         array.iter().sum::<i32>() % 10 == 0
//     }

//     fn is_odd(array: &[i32]) -> bool {
//         array.iter().sum::<i32>() % 2 == 1
//     }

//     fn helper(array: &[32], mul10: &[i32], odd: &[i32]) {
//         if array.len() == 0 {
//             mult_of_10(mul10) && is_odd(odd)
//         } else {
//             helper(&array[1..], array[0] + mul10, odd) || helper(&array[1..], mul10, array[0] + odd)
//             true
//         }
//     }
//     helper(array, [], []);
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_sum_test() {
        assert_eq!(group_sum(0, &[2, 4, 8], 10), true);
        assert_eq!(group_sum(0, &[2, 4, 8], 14), true);
        assert_eq!(group_sum(0, &[2, 4, 8], 9), false);
    }

    #[test]
    fn group_sum5_test() {
        assert_eq!(group_sum5(0, &[2, 5, 10, 4], 19), true);
        assert_eq!(group_sum5(0, &[2, 5, 10, 4], 17), true);
        assert_eq!(group_sum5(0, &[2, 5, 10, 4], 12), false);
        assert_eq!(group_sum5(0, &[2, 5, 10, 1, 4], 16), false);
        assert_eq!(group_sum5(0, &[2, 6, 10, 1, 4], 16), true);
    }

    #[test]
    fn split_odd10_test() {
        assert_eq!(split_odd10(&[5, 5, 5]), true);
        assert_eq!(split_odd10(&[5, 5, 6]), false);
        assert_eq!(split_odd10(&[5, 5, 6, 1]), true);
    }
}
