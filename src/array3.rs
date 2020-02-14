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

    fn search(a: &[u32], sub: &[u32]) -> bool {
        let (alen, sublen) = (a.len(), sub.len());
        for index in 0..=alen-sublen {
            if &a[index..index+sublen] == sub {
                return true;
            }
        }
        false
    }

    let alen = a.len();
    for length in (2..=alen).rev() {
        for leftx in 0..=alen-length {
            let reverse: Vec<u32> = a[leftx..leftx+length].iter().rev().cloned().collect();
            if search(a, &reverse) {
                return length as u32;
            }
        }
    }
    1 // this happens when there is no other match
}

// Array-3 > fix45
// https://codingbat.com/prob/p125819

// (This is a slightly harder version of the fix34 problem.)
// Return an array that contains exactly the same numbers as the given array,
// but rearranged so that every 4 is immediately followed by a 5.
// Do not move the 4's, but every other number may move.
// The array contains the same number of 4's and 5's, and every 4 has a number after it that is not a 4.
// In this version, 5's may appear anywhere in the original array.

// fix45([5, 4, 9, 4, 9, 5]) → [9, 4, 5, 4, 5, 9]
// fix45([1, 4, 1, 5]) → [1, 4, 5, 1]
// fix45([1, 4, 1, 5, 5, 4, 1]) → [1, 4, 5, 1, 1, 4, 5]

fn fix45(a: &[i32]) -> Vec<i32> {
    let mut avec = a.to_vec();
    let mut misplaced_5 = None; // index of a misplaced 5
    let mut ready2swap = None;  // the index of an element to swap with a 5
    for index in 0..avec.len() {
        if avec[index] == 5 {        // every 5 in the input is out of place
            misplaced_5 = Some(index);
        } else if avec[index] != 4 { // every element not 4 or 5 is ready to swap
            ready2swap = Some(index);
        }
        if misplaced_5 != None && ready2swap != None {
            // swap
            avec[misplaced_5.unwrap()] = avec[ready2swap.unwrap()];
            avec[ready2swap.unwrap()] = 5;
            misplaced_5 = None;
            ready2swap = None;
        }
    }
    avec
}

// Array-3 > squareUp
// https://codingbat.com/prob/p155405

// Given n>=0, create an array length n*n with the following pattern,
// shown here for n=3 : {0, 0, 1,    0, 2, 1,    3, 2, 1} (spaces added to show the 3 groups).

// square_up(3) → [0, 0, 1, 0, 2, 1, 3, 2, 1]
// square_up(2) → [0, 1, 2, 1]
// square_up(4) → [0, 0, 0, 1, 0, 0, 2, 1, 0, 3, 2, 1, 4, 3, 2, 1]

fn square_up(n: usize) -> Vec<usize> {
    let mut output = Vec::new();
    for group_number in 0..n {
        let mut group = vec![0; n];
        for index in 0..n {
            if index <= group_number {
                group[(n-1) - index] = index + 1;
            } else {
                group[(n-1) - index] = 0;
            }
        }
        output.append(&mut group);
    }
    output
}

// Array-3 > countClumps
// https://codingbat.com/prob/p193817

// Say that a "clump" in an array is a series of 2 or more adjacent elements of the same value.
// Return the number of clumps in the given array.

// count_clumps([1, 2, 2, 3, 4, 4]) → 2
// count_clumps([1, 1, 2, 1, 1]) → 2
// count_clumps([1, 1, 1, 1, 1]) → 1

fn count_clumps(array: &[u32]) -> u32 {
    let mut clumps = 0;
    let mut previous: Option<u32> = None;
    let mut repeat_counter = 1;
    for &element in array.iter() {
        match previous {
            None => previous = Some(element),
            Some(p) => {
                if element == p {
                    // the element is the same as previous
                    repeat_counter += 1;
                } else {
                    // the element is different than previous
                    repeat_counter = 1;
                    previous = Some(element);
                }
            }
        }
        if repeat_counter == 2 {
            clumps += 1;
        }
    }
    clumps
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
        assert_eq!(max_mirror(&[1, 2, 3, 2, 1]), 5);
        assert_eq!(max_mirror(&[1, 2, 3]), 1);
    }

    #[test]
    fn fix45_test() {
        assert_eq!(fix45(&[5, 4, 9, 4, 9, 5]), [9, 4, 5, 4, 5, 9]);
        assert_eq!(fix45(&[1, 4, 1, 5]), [1, 4, 5, 1]);
        assert_eq!(fix45(&[1, 4, 1, 5, 5, 4, 1]), [1, 4, 5, 1, 1, 4, 5]);
    }

    #[test]
    fn square_up_test() {
        assert_eq!(square_up(3), [0, 0, 1, 0, 2, 1, 3, 2, 1]);
        assert_eq!(square_up(2), [0, 1, 2, 1]);
        assert_eq!(square_up(4), [0, 0, 0, 1, 0, 0, 2, 1, 0, 3, 2, 1, 4, 3, 2, 1]);
    }

    #[test]
    fn count_clumps_test() {
        assert_eq!(count_clumps(&[1, 2, 2, 3, 4, 4]), 2);
        assert_eq!(count_clumps(&[1, 1, 2, 1, 1]), 2);
        assert_eq!(count_clumps(&[1, 1, 1, 1, 1]), 1);
    }
}
