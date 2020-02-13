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
}
