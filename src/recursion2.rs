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
// -> gs(start+1, array, target-array[start]) || gs(start+1, array, target) // either take it or don't

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

fn split_odd10(array: &[i32]) -> bool {

    fn mult_of_10(array: &[i32]) -> bool {
        array.iter().sum::<i32>() % 10 == 0
    }

    fn is_odd(array: &[i32]) -> bool {
        array.iter().sum::<i32>() % 2 == 1
    }

    fn helper(array: &[i32], mul10: &[i32], odd: &[i32]) -> bool {
        if array.len() == 0 {
            mult_of_10(mul10) && is_odd(odd)
        } else {
            let mut nextmul10 = mul10.to_vec();
            nextmul10.push(array[0]);

            let mut nextodd = odd.to_vec();
            nextodd.push(array[0]);

            helper(&array[1..], &nextmul10, odd) || helper(&array[1..], mul10, &nextodd)
        }
    }
    helper(array, &[], &[])
}

// Recursion-2 > groupSum6
// https://codingbat.com/prob/p199368

// Given an array of ints, is it possible to choose a group of some of the ints, beginning at the start index, such that the group sums to the given target? However, with the additional constraint that all 6's must be chosen. (No loops needed.)

// group_sum6(0, [5, 6, 2], 8) → true
// group_sum6(0, [5, 6, 2], 9) → false
// group_sum6(0, [5, 6, 2], 7) → false

fn group_sum6(start: usize, array: &[i32], target: i32) -> bool {
    if start == array.len() {
        target == 0
    } else {
        if array[start] == 6 {
            group_sum6(start+1, array, target - 6)
        } else {
            group_sum6(start+1, array, target - array[start]) || group_sum6(start+1, array, target)
        }
    }
}

// Recursion-2 > groupSumClump
// https://codingbat.com/prob/p105136

// Given an array of ints, is it possible to choose a group of some of the ints,
// such that the group sums to the given target, with this additional constraint:
// if there are numbers in the array that are adjacent and the identical value,
// they must either all be chosen, or none of them chosen.
// For example, with the array {1, 2, 2, 2, 5, 2}, either all three 2's in the middle must be chosen or not, all as a group.

// group_sum_clump(0, [2, 4, 8], 10) → true
// group_sum_clump(0, [1, 2, 4, 8, 1], 14) → true
// group_sum_clump(0, [2, 4, 4, 8], 14) → false

fn group_sum_clump(start: usize, array: &[i32], target: i32) -> bool {

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

    fn helper(start: usize, array: &[i32], target: i32) -> bool {
        if start == array.len() {
            target == 0
        } else {
            let clump_length = clump_length(start, array);
            let clump_sum = array[start] * (clump_length as i32);
            helper(start+clump_length, array, target-clump_sum) || helper(start+clump_length, array, target)
        }
    }
    helper(start, array, target)
}

// Recursion-2 > split53
// https://codingbat.com/prob/p168295

// Given an array of ints, is it possible to divide the ints into two groups,
// so that the sum of the two groups is the same, with these constraints:
// all the values that are multiple of 5 must be in one group,
// and all the values that are a multiple of 3 (and not a multiple of 5) must be in the other.

// split53([1, 1]) → true
// split53([1, 1, 1]) → false
// split53([2, 4, 2]) → true

fn split53(array: &[i32]) -> bool {

    fn helper(array: &[i32], mult5: &[i32], mult3: &[i32]) -> bool {
        if array.len() == 0 {
            mult5.iter().sum::<i32>() == mult3.iter().sum::<i32>()
        } else {
            let head = array.iter().next().unwrap().clone();

            let mut mult5next = mult5.to_vec();
            mult5next.push(head);

            let mut mult3next = mult3.to_vec();
            mult3next.push(head);

            if head % 5 == 0 {
                helper(&array[1..], &mult5next, mult3)
            } else if head % 3 == 0 {
                helper(&array[1..], mult5, &mult3next)
            } else {
                helper(&array[1..], &mult5next, mult3) || helper(&array[1..], mult5, &mult3next)
            }
        }
    }
    helper(array, &[], &[])
}

// Recursion-2 > groupNoAdj
// https://codingbat.com/prob/p169605

// Given an array of ints, is it possible to choose a group of some of the ints,
// such that the group sums to the given target with this additional constraint:
// If a value in the array is chosen to be in the group,
// the value immediately following it in the array must not be chosen. (No loops needed.)

// group_no_adj(0, [2, 5, 10, 4], 12) → true
// group_no_adj(0, [2, 5, 10, 4], 14) → false
// group_no_adj(0, [2, 5, 10, 4], 7) → false

fn group_no_adj(start: usize, array: &[i32], target: i32) -> bool {
    if start >= array.len() {
        target == 0
    } else {
        group_no_adj(start+2, array, target - array[start]) || group_no_adj(start+1, array, target)
    }
}

// Recursion-2 > splitArray
// https://codingbat.com/prob/p185204

// Given an array of ints,
// is it possible to divide the ints into two groups, so that the sums of the two groups are the same.
// Every int must be in one group or the other.

// split_array([2, 2]) → true
// split_array([2, 3]) → false
// split_array([5, 2, 3]) → true

fn split_array(array: &[i32]) -> bool {

    fn helper(array: &[i32], left: &[i32], right: &[i32]) -> bool {
        if array.len() == 0 {
            left.iter().sum::<i32>() == right.iter().sum::<i32>()
        } else {
            let head = array.iter().next().unwrap().clone();

            let mut nextleft = left.to_vec();
            nextleft.push(head);

            let mut nextright = right.to_vec();
            nextright.push(head);

            helper(&array[1..], &nextleft, right) || helper(&array[1..], left, &nextright)
        }
    }
    helper(array, &[], &[])
}

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

    #[test]
    fn group_sum6_test() {
        assert_eq!(group_sum6(0, &[5, 6, 2], 8), true);
        assert_eq!(group_sum6(0, &[5, 6, 2], 9), false);
        assert_eq!(group_sum6(0, &[5, 6, 2], 7), false);
    }

    #[test]
    fn group_sum_clump_test() {
        assert_eq!(group_sum_clump(0, &[2, 4, 8], 10), true);
        assert_eq!(group_sum_clump(0, &[1, 2, 4, 8, 1], 14), true);
        assert_eq!(group_sum_clump(0, &[2, 4, 4, 8], 14), false);
        assert_eq!(group_sum_clump(0, &[3,3,5], 8), false); // 5 + 3 == 8, but both 3s must be chosen
    }

    #[test]
    fn split53_test() {
        assert_eq!(split53(&[1, 1]), true);
        assert_eq!(split53(&[1, 1, 1]), false);
        assert_eq!(split53(&[2, 4, 2]), true);
    }

    #[test]
    fn group_no_adj_test() {
        assert_eq!(group_no_adj(0, &[2, 5, 10, 4], 12), true);
        assert_eq!(group_no_adj(0, &[2, 5, 10, 4], 14), false);
        assert_eq!(group_no_adj(0, &[2, 5, 10, 4], 7), false);
    }

    #[test]
    fn split_array_test() {
        assert_eq!(split_array(&[2, 2]), true);
        assert_eq!(split_array(&[2, 3]), false);
        assert_eq!(split_array(&[5, 2, 3]), true);
    }
}
