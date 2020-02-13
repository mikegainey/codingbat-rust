// Array-3 > maxSpan
// https://codingbat.com/prob/p189576

// Consider the leftmost and righmost appearances of some value in an array.
// We'll say that the "span" is the number of elements between the two inclusive.
// A single value has a span of 1. Return the largest span found in the given array. (Efficiency is not a priority.)

// max_span([1, 2, 1, 1, 3]) → 4
// max_span([1, 4, 2, 1, 4, 1, 4]) → 6
// max_span([1, 4, 2, 1, 4, 4, 4]) → 6

fn max_span(a: &[i32]) -> usize {
    let alen = a.len();
    let mut rightx;
    for length in (1..=alen).rev() {   // loop through possible lengths
        for leftx in 0..=alen-length { // loop through possible left indexes
            rightx = leftx + length - 1;
            if a[leftx] == a[rightx] {
                let span = rightx - leftx + 1;
                return span;
            }
        }
    }
    0 // this should never happen because every number is it's own span of 1
}

// Array-3 > canBalance
// https://codingbat.com/prob/p158767

// Given a non-empty array, return true if there is a place to split the array
// so that the sum of the numbers on one side is equal to the sum of the numbers on the other side.

// can_balance([1, 1, 1, 2, 1]) → true
// can_balance([2, 1, 1, 2, 1]) → false
// can_balance([10, 10]) → true

fn can_balance(a: &[i32]) -> bool {
    let sum: i32 = a.iter().sum();
    if sum % 2 == 1 {
        return false;
    }
    let half_sum = sum / 2;
    let mut partial_sum = 0;
    for element in a.iter() {
        partial_sum += element;
        if partial_sum == half_sum {
            return true;
        }
    }
    false
}

// Array-3 > seriesUp
// https://codingbat.com/prob/p104090

// Given n>=0, create an array with the pattern {1,    1, 2,    1, 2, 3,   ... 1, 2, 3 .. n}
// (spaces added to show the grouping).
// Note that the length of the array will be 1 + 2 + 3 ... + n, which is known to sum to exactly n*(n + 1)/2.

// series_up(3) → [1, 1, 2, 1, 2, 3]
// series_up(4) → [1, 1, 2, 1, 2, 3, 1, 2, 3, 4]
// series_up(2) → [1, 1, 2]

fn series_up(n: i32) -> Vec<i32> {
    let mut output = Vec::new();
    for end in 1..=n {
        for inner in 1..=end {
            output.push(inner);
        }
    }
    output
}

// Array-3 > fix34
// https://codingbat.com/prob/p159339

// Return an array that contains exactly the same numbers as the given array,
// but rearranged so that every 3 is immediately followed by a 4.
// Do not move the 3's, but every other number may move.
// The array contains the same number of 3's and 4's,
// every 3 has a number after it that is not a 3, and a 3 appears in the array before any 4.

// fix34([1, 3, 1, 4]) → [1, 3, 4, 1]
// fix34([1, 3, 1, 4, 4, 3, 1]) → [1, 3, 4, 1, 1, 3, 4]
// fix34([3, 2, 2, 4]) → [3, 4, 2, 2]

fn fix34(array: &[i32]) -> Vec<i32> {
    let mut threes = Vec::new();
    let mut others = Vec::new();
    let mut four_count = 0;

    // find the indices of the 3s, count the 4s, list all the other numbers
    for (index, &a) in array.iter().enumerate() {
        if a == 3 {
            threes.push(index);
        } else if a == 4 {
            four_count += 1;
        } else {
            others.push(a)
        }
    }
    assert_eq!(threes.len(), four_count);

    // use others, three_x, and our_count to create the output
    let mut output = Vec::new();
    let mut index = 0;
    let mut others_iter = others.iter();
    while index < array.len() {
        if threes.contains(&index) {
            output.push(3);
            output.push(4);
            index += 2;
        } else {
            output.push(*others_iter.next().unwrap());
            index += 1;
        }
    }
    output
}

// Array-3 > linearIn
// https://codingbat.com/prob/p134022

// Given two arrays of ints sorted in increasing order, outer and inner,
// return true if all of the numbers in inner appear in outer.
// The best solution makes only a single "linear" pass of both arrays,
// taking advantage of the fact that both arrays are already in sorted order.

// linear_in([1, 2, 4, 6], [2, 4]) → true
// linear_in([1, 2, 4, 6], [2, 3, 4]) → false
// linear_in([1, 2, 4, 4, 6], [2, 4]) → true

fn linear_in(outer: &[i32], inner: &[i32]) -> bool {
    let mut ox = 0;
    let mut ix = 0;
    loop {
        if outer[ox] == inner[ix] {
            ix += 1;
            ox += 1;
        } else {
            ox += 1;
        }
        // if the end of inner is reached, success!
        if ix >= inner.len() {
            return true;
        }
        // if the end of outer is reached (before the end of inner), failure!
        if ox >= outer.len() {
            return false;
        }
    }
}

// Array-3 > maxMirror
// https://codingbat.com/prob/p196409

// We'll say that a "mirror" section in an array is a group of contiguous elements such that somewhere in the array,
// the same group appears in reverse order.
// For example, the largest mirror section in {1, 2, 3, 8, 9, 3, 2, 1} is length 3 (the {1, 2, 3} part).
// Return the size of the largest mirror section found in the given array.

// max_mirror([1, 2, 3, 8, 9, 3, 2, 1]) → 3
// max_mirror([1, 2, 1, 4]) → 3
// max_mirror([7, 1, 2, 9, 7, 2, 1]) → 2

fn max_mirror(a: &[u32]) -> u32 {
    a[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_span_test() {
        assert_eq!(max_span(&[1, 2, 1, 1, 3]), 4);
        assert_eq!(max_span(&[1, 4, 2, 1, 4, 1, 4]), 6);
        assert_eq!(max_span(&[1, 4, 2, 1, 4, 4, 4]), 6);
    }

    #[test]
    fn can_balance_test() {
        assert_eq!(can_balance(&[1, 1, 1, 2, 1]), true);
        assert_eq!(can_balance(&[2, 1, 1, 2, 1]), false);
        assert_eq!(can_balance(&[10, 10]), true);
    }

    #[test]
    fn series_up_test() {
        assert_eq!(series_up(3), [1, 1, 2, 1, 2, 3]);
        assert_eq!(series_up(4), [1, 1, 2, 1, 2, 3, 1, 2, 3, 4]);
        assert_eq!(series_up(2), [1, 1, 2]);
    }

    #[test]
    fn fix34_test() {
        assert_eq!(fix34(&[1, 3, 1, 4]), [1, 3, 4, 1]);
        assert_eq!(fix34(&[1, 3, 1, 4, 4, 3, 1]), [1, 3, 4, 1, 1, 3, 4]);
        assert_eq!(fix34(&[3, 2, 2, 4]), [3, 4, 2, 2]);
    }

    #[test]
    fn linear_in_test() {
        assert_eq!(linear_in(&[1, 2, 4, 6], &[2, 4]), true);
        assert_eq!(linear_in(&[1, 2, 4, 6], &[2, 3, 4]), false);
        assert_eq!(linear_in(&[1, 2, 4, 4, 6], &[2, 4]), true);
        assert_eq!(linear_in(&[1, 2, 4, 4, 6], &[2, 4, 6]), true);
    }

    #[test]
    fn max_mirror_test() {
        assert_eq!(max_mirror(&[1, 2, 3, 8, 9, 3, 2, 1]), 3);
        assert_eq!(max_mirror(&[1, 2, 1, 4]), 3);
        assert_eq!(max_mirror(&[7, 1, 2, 9, 7, 2, 1]), 2);
    }
}
