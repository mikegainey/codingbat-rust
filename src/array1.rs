// Array-1 > firstLast6
// https://codingbat.com/prob/p185685

// Given an array of ints, return true if 6 appears as either the first or last element in the array.
// The array will be length 1 or more.

// first_last6([1, 2, 6]) → true
// first_last6([6, 1, 2, 3]) → true
// first_last6([13, 6, 1, 2, 3]) → false

fn first_last6(a: &[i32]) -> bool {
    let len = a.len();
    assert!(len >= 1, "The input array was too short.");
    a[0] == 6 || a[len-1] == 6
}

// Array-1 > commonEnd
// https://codingbat.com/prob/p191991

// Given 2 arrays of ints, a and b, return true if they have the same first element or they have the same last element.
// Both arrays will be length 1 or more.

// common_end([1, 2, 3], [7, 3]) → true
// common_end([1, 2, 3], [7, 3, 2]) → false
// common_end([1, 2, 3], [1, 3]) → true

fn common_end(a: &[i32], b: &[i32]) -> bool {
    a[0] == b[0] || a[a.len()-1] == b[b.len()-1]
}

// Array-1 > reverse3
// https://codingbat.com/prob/p112409

// Given an array of ints length 3, return a new array with the elements in reverse order, so {1, 2, 3} becomes {3, 2, 1}.

// reverse3([1, 2, 3]) → [3, 2, 1]
// reverse3([5, 11, 9]) → [9, 11, 5]
// reverse3([7, 0, 0]) → [0, 0, 7]

fn reverse3a(a: [i32; 3]) -> [i32; 3] {
    let mut output = [0; 3]; // an array with 3 zeros
    for x in 0..3 {
        output[x] = a[2-x];
    }
    output
}

fn reverse3b(a: [i32; 3]) -> Vec<i32> {
    let mut output = Vec::new();
    for element in a.iter().rev() {
        output.push(*element);
    }
    output
}

fn reverse3c(a: [i32; 3]) -> Vec<i32> {
    a.iter().rev()
        .map(|&x| x) // to convert from an array of references to an array of values
        .collect::<Vec<_>>()
}

// Array-1 > middleWay
// https://codingbat.com/prob/p146449

// Given 2 int arrays, a and b, each length 3, return a new array length 2 containing their middle elements.

// middle_way([1, 2, 3], [4, 5, 6]) → [2, 5]
// middle_way([7, 7, 7], [3, 8, 0]) → [7, 8]
// middle_way([5, 2, 9], [1, 4, 5]) → [2, 4]

fn middle_way(a: [i32; 3], b: [i32; 3]) -> [i32; 2] {
    let mut output = [0, 0];
    output[0] = a[1];
    output[1] = b[1];
    output
}
// Array-1 > no23
// https://codingbat.com/prob/p175689

// Given an int array length 2, return true if it does not contain a 2 or 3.

// no23([4, 5]) → true
// no23([4, 2]) → false
// no23([3, 5]) → false

fn no23(a: [i32; 2]) -> bool {
    a.iter()
        .all(|&x| x != 2 && x != 3)
}

// Array-1 > fix23
// https://codingbat.com/prob/p120347

// Given an int array length 3, if there is a 2 in the array immediately followed by a 3, set the 3 element to 0.
// Return the changed array.

// fix23([1, 2, 3]) → [1, 2, 0]
// fix23([2, 3, 5]) → [2, 0, 5]
// fix23([1, 2, 1]) → [1, 2, 1]

fn fix23(mut a: [i32; 3]) -> [i32; 3] {
    if a[0..2] == [2,3] {
        a[1] = 0;
    }
    if a[1..3] == [2,3] {
        a[2] = 0;
    }
    a
}

// Array-1 > makeMiddle
// https://codingbat.com/prob/p199519

// Given an array of ints of even length,
// return a new array length 2 containing the middle two elements from the original array.
// The original array will be length 2 or more.

// make_middle([1, 2, 3, 4]) → [2, 3]
// make_middle([7, 1, 2, 3, 4, 9]) → [2, 3]
// make_middle([1, 2]) → [1, 2]

fn make_middle(a: &[i32]) -> [i32; 2] {
    let len = a.len();
    assert!(len % 2 == 0); // ensure the input is of even length
    let midx = len / 2;    // midx is the index of the first element of the second half
    [a[midx-1], a[midx]]
}

// Array-1 > midThree
// https://codingbat.com/prob/p155713

// Given an array of ints of odd length, return a new array length 3 containing the elements from the middle of the array.
// The array length will be at least 3.

// mid_three([1, 2, 3, 4, 5]) → [2, 3, 4]
// mid_three([8, 6, 7, 5, 3, 0, 9]) → [7, 5, 3]
// mid_three([1, 2, 3]) → [1, 2, 3]

fn mid_three(a: &[i32]) -> [i32; 3] {
    use std::convert::TryInto;

    let len = a.len();
    assert!(len >= 3 && len % 2 == 1); // ensure the input is of odd length with at least 3 elements
    let i = len / 2 - 1;
    let output = &a[i..i+3];
    output.try_into().unwrap()
}

// Array-1 > unlucky1
// https://codingbat.com/prob/p197308

// We'll say that a 1 immediately followed by a 3 in an array is an "unlucky" 1.
// Return true if the given array contains an unlucky 1 in the first 2 or last 2 positions in the array.

// unlucky1([1, 3, 4, 5]) → true
// unlucky1([2, 1, 3, 4, 5]) → true
// unlucky1([1, 1, 1]) → false

fn unlucky1(a: &[i32]) -> bool {
    let len = a.len();

    let first1 = &a[0..2] == [1,3];
    let first2 = &a[1..3] == [1,3];
    let last2 = &a[len-2..len] == [1,3];

    first1 || first2 || last2
}

// Array-1 > sameFirstLast
// https://codingbat.com/prob/p118976

// Given an array of ints, return true if the array is length 1 or more, and the first element and the last element are equal.

// same_first_last([1, 2, 3]) → false
// same_first_last([1, 2, 3, 1]) → true
// same_first_last([1, 2, 1]) → true

fn same_first_last(a: &[i32]) -> bool {
    let len = a.len();

    if len <= 1 {
        return false;
    } else {
        a[0] == a[len-1]
    }
}

// Array-1 > sum3
// https://codingbat.com/prob/p175763

// Given an array of ints length 3, return the sum of all the elements.

// sum3([1, 2, 3]) → 6
// sum3([5, 11, 2]) → 18
// sum3([7, 0, 0]) → 7

fn sum3(a: [i32; 3]) -> i32 {
    a.iter().sum()
}

// Array-1 > maxEnd3
// https://codingbat.com/prob/p146256

// Given an array of ints length 3, figure out which is larger, the first or last element in the array,
// and set all the other elements to be that value. Return the changed array.

// max_end3([1, 2, 3]) → [3, 3, 3]
// max_end3([11, 5, 9]) → [11, 11, 11]
// max_end3([2, 11, 3]) → [3, 3, 3]

fn max_end3(mut a: [i32; 3]) -> [i32; 3] {
    use std::cmp;

    let larger_value = cmp::max(a[0], a[2]);
    a[0] = larger_value;
    a[1] = larger_value;
    a[2] = larger_value;
    a
}

// Array-1 > makeEnds
// https://codingbat.com/prob/p101230

// Given an array of ints, return a new array length 2 containing the first and last elements from the original array.
// The original array will be length 1 or more.

// make_ends([1, 2, 3]) → [1, 3]
// make_ends([1, 2, 3, 4]) → [1, 4]
// make_ends([7, 4, 6, 2]) → [7, 2]

fn make_ends(a: &[i32]) -> [i32; 2] {
    let len = a.len();
    [a[0], a[len-1]]
}

// Array-1 > makeLast
// https://codingbat.com/prob/p137188

// Given an int array, return a new array with double the length where its last element is the same as the original array,
// and all the other elements are 0. The original array will be length 1 or more.
// Note: by default, a new int array contains all 0's.

// make_last([4, 5, 6]) → [0, 0, 0, 0, 0, 6]
// make_last([1, 2]) → [0, 0, 0, 2]
// make_last([3]) → [0, 3]

fn make_last(a: &[i32]) -> &[i32] {
    let len = a.len();
    let mut output = [0; len * 2];
    output[(len*2)-1] = a[len-1];
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_last6_test() {
        assert_eq!(first_last6(&[1, 2, 6]), true);
        assert_eq!(first_last6(&[6, 1, 2, 3]), true);
        assert_eq!(first_last6(&[13, 6, 1, 2, 3]), false);
    }

    #[test]
    fn common_end_test() {
        assert_eq!(common_end(&[1, 2, 3], &[7, 3]), true);
        assert_eq!(common_end(&[1, 2, 3], &[7, 3, 2]), false);
        assert_eq!(common_end(&[1, 2, 3], &[1, 3]), true);
    }

    #[test]
    fn reverse3_test() {
        assert_eq!(reverse3a([1, 2, 3]), [3, 2, 1]);
        assert_eq!(reverse3a([5, 11, 9]), [9, 11, 5]);
        assert_eq!(reverse3a([7, 0, 0]), [0, 0, 7]);
        assert_eq!(reverse3b([1, 2, 3]), [3, 2, 1]);
        assert_eq!(reverse3b([5, 11, 9]), [9, 11, 5]);
        assert_eq!(reverse3b([7, 0, 0]), [0, 0, 7]);
        assert_eq!(reverse3c([1, 2, 3]), [3, 2, 1]);
        assert_eq!(reverse3c([5, 11, 9]), [9, 11, 5]);
        assert_eq!(reverse3c([7, 0, 0]), [0, 0, 7]);
    }

    #[test]
    fn middle_way_test() {
        assert_eq!(middle_way([1, 2, 3], [4, 5, 6]), [2, 5]);
        assert_eq!(middle_way([7, 7, 7], [3, 8, 0]), [7, 8]);
        assert_eq!(middle_way([5, 2, 9], [1, 4, 5]), [2, 4]);
    }

    #[test]
    fn no23_test() {
        assert_eq!(no23([4, 5]), true);
        assert_eq!(no23([4, 2]), false);
        assert_eq!(no23([3, 5]), false);
    }

    #[test]
    fn fix23_test() {
        assert_eq!(fix23([1, 2, 3]), [1, 2, 0]);
        assert_eq!(fix23([2, 3, 5]), [2, 0, 5]);
        assert_eq!(fix23([1, 2, 1]), [1, 2, 1]);
    }

    #[test]
    fn make_middle_test() {
        assert_eq!(make_middle(&[1, 2, 3, 4]), [2, 3]);
        assert_eq!(make_middle(&[7, 1, 2, 3, 4, 9]), [2, 3]);
        assert_eq!(make_middle(&[1, 2]), [1, 2]);
    }

    #[test]
    fn mid_three_test() {
        assert_eq!(mid_three(&[1, 2, 3, 4, 5]), [2, 3, 4]);
        assert_eq!(mid_three(&[8, 6, 7, 5, 3, 0, 9]), [7, 5, 3]);
        assert_eq!(mid_three(&[1, 2, 3]), [1, 2, 3]);
    }

    #[test]
    fn unlucky1_test() {
        assert_eq!(unlucky1(&[1, 3, 4, 5]), true);
        assert_eq!(unlucky1(&[2, 1, 3, 4, 5]), true);
        assert_eq!(unlucky1(&[1, 1, 1]), false);
        assert_eq!(unlucky1(&[1, 1, 1, 3]), true);
    }

    #[test]
    fn same_first_last_test() {
        assert_eq!(same_first_last(&[1, 2, 3]), false);
        assert_eq!(same_first_last(&[1, 2, 3, 1]), true);
        assert_eq!(same_first_last(&[1, 2, 1]), true);
    }

    #[test]
    fn sum3_test() {
        assert_eq!(sum3([1, 2, 3]), 6);
        assert_eq!(sum3([5, 11, 2]), 18);
        assert_eq!(sum3([7, 0, 0]), 7);
    }

    #[test]
    fn max_end3_test() {
        assert_eq!(max_end3([1, 2, 3]), [3, 3, 3]);
        assert_eq!(max_end3([11, 5, 9]), [11, 11, 11]);
        assert_eq!(max_end3([2, 11, 3]), [3, 3, 3]);
    }

    #[test]
    fn make_ends_test() {
        assert_eq!(make_ends(&[1, 2, 3]), [1, 3]);
        assert_eq!(make_ends(&[1, 2, 3, 4]), [1, 4]);
        assert_eq!(make_ends(&[7, 4, 6, 2]), [7, 2]);
    }

    #[test]
    fn make_last_test() {
        assert_eq!(make_last(&[4, 5, 6]), [0, 0, 0, 0, 0, 6]);
        assert_eq!(make_last(&[1, 2]), [0, 0, 0, 2]);
        assert_eq!(make_last(&[3]), [0, 3]);
    }
}
