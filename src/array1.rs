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
fn reverse3a(a: &[i32]) -> [i32; 3] {
    let mut output = [0, 0, 0];
    for x in 0..3 {
        output[x] = a[2-x];
    }
    output
}

fn reverse3b(a: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    for element in a.iter().rev() {
        output.push(*element);
    }
    output
}

fn reverse3c(a: &[i32]) -> Vec<i32> {
    a.iter().rev()
        .map(|&x| x) // is this the idiomatic way of doing this?
        .collect::<Vec<_>>()
}

// Array-1 > middleWay
// https://codingbat.com/prob/p146449

// Given 2 int arrays, a and b, each length 3, return a new array length 2 containing their middle elements.

// middle_way([1, 2, 3], [4, 5, 6]) → [2, 5]
// middle_way([7, 7, 7], [3, 8, 0]) → [7, 8]
// middle_way([5, 2, 9], [1, 4, 5]) → [2, 4]

fn middle_way(a: &[i32], b: &[i32]) -> [i32; 2] {
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

fn no23(a: &[i32]) -> bool {
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

fn fix23(a: &[i32]) -> Vec<i32> {
    a.to_vec()
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
        assert_eq!(reverse3a(&[1, 2, 3]), [3, 2, 1]);
        assert_eq!(reverse3a(&[5, 11, 9]), [9, 11, 5]);
        assert_eq!(reverse3a(&[7, 0, 0]), [0, 0, 7]);
        assert_eq!(reverse3b(&[1, 2, 3]), [3, 2, 1]);
        assert_eq!(reverse3b(&[5, 11, 9]), [9, 11, 5]);
        assert_eq!(reverse3b(&[7, 0, 0]), [0, 0, 7]);
        assert_eq!(reverse3c(&[1, 2, 3]), [3, 2, 1]);
        assert_eq!(reverse3c(&[5, 11, 9]), [9, 11, 5]);
        assert_eq!(reverse3c(&[7, 0, 0]), [0, 0, 7]);
    }

    #[test]
    fn middle_way_test() {
        assert_eq!(middle_way(&[1, 2, 3], &[4, 5, 6]), [2, 5]);
        assert_eq!(middle_way(&[7, 7, 7], &[3, 8, 0]), [7, 8]);
        assert_eq!(middle_way(&[5, 2, 9], &[1, 4, 5]), [2, 4]);
    }

    #[test]
    fn no23_test() {
        assert_eq!(no23(&[4, 5]), true);
        assert_eq!(no23(&[4, 2]), false);
        assert_eq!(no23(&[3, 5]), false);
    }

    #[test]
    fn fix23_test() {
        assert_eq!(fix23(&[1, 2, 3]), [1, 2, 0]);
        assert_eq!(fix23(&[2, 3, 5]), [2, 0, 5]);
        assert_eq!(fix23(&[1, 2, 1]), [1, 2, 1]);
    }
}
