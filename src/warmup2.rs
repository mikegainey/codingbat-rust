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

fn last2(s: &str) -> u32 {
    let len = s.len();
    let substring = &s[len-2..];
    let search_string = &s[..len-2];
    0
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
        assert_eq!(last2("hixxhi"), 1);
        assert_eq!(last2("xaxxaxaxx"), 1);
        assert_eq!(last2("axxxaaxx"), 2);
    }
}
