// Warmup-2 > stringTimes
// https://codingbat.com/prob/p142270

// Given a string and a non-negative int n, return a larger string that is n copies of the original string.

// string_times("Hi", 2) → "HiHi"
// string_times("Hi", 3) → "HiHiHi"
// string_times("Hi", 1) → "Hi"

fn string_times(s: &str, n: usize) -> String {
    s.repeat(n)
}

// Warmup-2 > doubleX
// https://codingbat.com/prob/p186759

// Given a string, return true if the first instance of "x" in the string is immediately followed by another "x".

// double_x("axxbb") → true
// double_x("axaxax") → false
// double_x("xxxxx") → true

fn double_x(s: &str) -> bool {
    let mut previous_char = ' '; // this assumes the input string doesn't contain spaces

    for c in s.chars() {
        if (c == 'x') && (previous_char == 'x') {
            return true
        }
        previous_char = c;
    }
    false
}

// Warmup-2 > last2
// https://codingbat.com/prob/p178318

// Given a string, return the count of the number of times that a substring length 2 appears in the string
// and also as the last 2 chars of the string, so "hixxxhi" yields 1 (we won't count the end substring).

// last2("hixxhi") → 1
// last2("xaxxaxaxx") → 1
// last2("axxxaaxx") → 2

fn last2a(s: &str) -> u32 {
    let len = s.len();
    let substring = &s[len-2..];
    let search_string = &s[..len-2];

    let mut count = 0;
    for x in 0..=search_string.len()-2 {
        if &search_string[x..x+2] == substring {
            count += 1
        }
    }
    count
}

fn last2b(s: &str) -> u32 {
    let len = s.len();
    let substring = &s[len-2..];
    let search_string = &s[..len-2];

    fn substring_count(search_string: &str, substring: &str) -> u32 {
        if search_string.len() < 2 {
            0
        } else {
            if &search_string[0..2] == substring {
                1 + substring_count(&search_string[1..], &substring)
            } else {
                0 + substring_count(&search_string[1..], &substring)
            }
        }
    }

    substring_count(&search_string, &substring)
}

// Warmup-2 > array123
// https://codingbat.com/prob/p136041

// Given an array of ints, return true if the sequence of numbers 1, 2, 3 appears in the array somewhere.

// array123([1, 1, 2, 3, 1]) → true
// array123([1, 1, 2, 4, 1]) → false
// array123([1, 1, 2, 1, 2, 3]) → true

fn array123a(a: Vec<i32>) -> bool {
    let len = a.len();
    for x in 0..=len-3 {
        if &a[x..x+3] == &[1,2,3] {
            return true
        }
    }
    false
}

fn array123b(a: &Vec<i32>) -> bool {
    if a.len() < 3 {
        return false
    } else {
        return (&a[0..3] == &[1,2,3]) || array123b(&a[1..])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_times_test() {
        assert_eq!(string_times("Hi", 2), "HiHi");
        assert_eq!(string_times("Hi", 3), "HiHiHi");
        assert_eq!(string_times("Hi", 1), "Hi");
    }

    #[test]
    fn double_x_test() {
        assert_eq!(double_x("axxbb"), true);
        assert_eq!(double_x("axaxax"), false);
        assert_eq!(double_x("xxxxx"), true);
    }

    #[test]
    fn last2_test() {
        assert_eq!(last2a("hixxhi"), 1);
        assert_eq!(last2a("xaxxaxaxx"), 1);
        assert_eq!(last2a("axxxaaxx"), 2);
        assert_eq!(last2a("michmiaemimi"), 3);
        assert_eq!(last2b("hixxhi"), 1);
        assert_eq!(last2b("xaxxaxaxx"), 1);
        assert_eq!(last2b("axxxaaxx"), 2);
        assert_eq!(last2b("michmiaemimi"), 3);
    }

    #[test]
    fn array123_test() {
        assert_eq!(array123a(vec![1, 1, 2, 3, 1]), true);
        assert_eq!(array123a(vec![1, 1, 2, 4, 1]), false);
        assert_eq!(array123a(vec![1, 1, 2, 1, 2, 3]), true);
        assert_eq!(array123a(vec![1, 2, 3, 1, 2, 4]), true);
        // assert_eq!(array123b(vec![1, 1, 2, 3, 1]), true);
        // assert_eq!(array123b(vec![1, 1, 2, 4, 1]), false);
        // assert_eq!(array123b(vec![1, 1, 2, 1, 2, 3]), true);
        // assert_eq!(array123b(vec![1, 2, 3, 1, 2, 4]), true);
    }
}
