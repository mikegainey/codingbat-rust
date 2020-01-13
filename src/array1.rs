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

// there has to be a better way ...
fn reverse3(a: &[i32]) -> [i32; 3] {
    let mut output = [0, 0, 0];
    for x in 0..3 {
        output[x] = a[2-x];
    }
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
        assert_eq!(reverse3(&[1, 2, 3]), [3, 2, 1]);
        assert_eq!(reverse3(&[5, 11, 9]), [9, 11, 5]);
        assert_eq!(reverse3(&[7, 0, 0]), [0, 0, 7]);
    }
}
