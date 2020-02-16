// String-3 > countYZ
// https://codingbat.com/prob/p199171

// Given a string, count the number of words ending in 'y' or 'z'
// -- so the 'y' in "heavy" and the 'z' in "fez" count, but not the 'y' in "yellow" (not case sensitive).
// We'll say that a y or z is at the end of a word if there is not an alphabetic letter immediately following it.

// count_yz("fez day") → 2
// count_yz("day fez") → 2
// count_yz("day fyyyz") → 2

fn count_yz(s: &str) -> usize {
    let words: Vec<&str> = s.split(' ').collect();
    words.iter()
        .filter(|w|
                w.chars().last().unwrap() == 'y' ||
                w.chars().last().unwrap() == 'z')
        .count()
}

// String-3 > gHappy
// https://codingbat.com/prob/p198664

// We'll say that a lowercase 'g' in a string is "happy" if there is another 'g' immediately to its left or right.
// Return true if all the g's in the given string are happy.

// g_happy("xxggxx") → true
// g_happy("xxgxx") → false
// g_happy("xxggyygxx") → false

// fails on a lone g at the end

fn g_happy(s: &str) -> bool {
    let mut g_count = 0;
    for c in s.chars() {
        if c == 'g' {
            // g_count consecutive 'g's
            g_count += 1;
        } else {
            // check if a non-g follows a lone g (unhappy)
            if g_count == 1 {
                return false;
            } else {
                g_count = 0;
            }
        }
    }
    // check for a lone g at the end
    if g_count == 1 {
        false
    } else {
        true
    }
}

// String-3 > sameEnds
// https://codingbat.com/prob/p131516

// Given a string, return the longest substring that appears at both the beginning and end of the string without overlapping.
// For example, sameEnds("abXab") is "ab".

// same_ends("abXYab") → "ab"
// same_ends("xx") → "x"
// same_ends("xxx") → "x"

fn same_ends(s: &str) -> &str {
    let strlen = s.len();
    let maxlen = s.len() / 2;
    for sublen in (1..=maxlen).rev() {
        if s[..sublen] == s[strlen-sublen..] {
            return &s[..sublen];
        }
    }
    ""
}

// String-3 > sumNumbers
// https://codingbat.com/prob/p121193

// Given a string, return the sum of the numbers appearing in the string, ignoring all other characters.
// A number is a series of 1 or more digit chars in a row.

// sum_numbers("abc123xyz") → 123
// sum_numbers("aa11b33") → 44
// sum_numbers("7 11") → 18

fn sum_numbers(s: &str) -> u32 {
    let mut number_list = Vec::new();
    let mut left: Option<usize> = None;  // indices of numbers
    let mut right: Option<usize> = None; // indices of numbers
    let mut number: u32;
    for (index, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            if left == None {
                left = Some(index);
                right = Some(index);
            } else {
                right = Some(index);
            }
        } else {
            // char is not a digit
            if left != None && right != None {
                number = s[left.unwrap()..=right.unwrap()].parse().unwrap();
                number_list.push(number);
                left = None;
                right = None;
            }
        }
    }
    // for a number at the end of the string
    if left != None && right != None {
        number = s[left.unwrap()..=right.unwrap()].parse().unwrap();
        number_list.push(number);
    }
    number_list.iter().sum()
}

// String-3 > withoutString
// https://codingbat.com/prob/p192570

// Given two strings, base and remove,
// return a version of the base string where all instances of the remove string have been removed (not case sensitive).
// You may assume that the remove string is length 1 or more.
// Remove only non-overlapping instances, so with "xxx" removing "xx" leaves "x".

// without_string("Hello there", "llo") → "He there"
// without_string("Hello there", "e") → "Hllo thr"
// without_string("Hello there", "x") → "Hello there"

fn without_string(base: &str, remove: &str) -> String {
    let remlen = remove.len();
    if base.len() < remlen {
        base.to_string()
    } else {
        if base[..remlen].to_lowercase() == remove.to_lowercase() {
            without_string(&base[remlen..], remove)
        } else {
            format!("{}{}", &base[..1], without_string(&base[1..], remove))
        }
    }
}

// String-3 > countTriple
// https://codingbat.com/prob/p195714

// We'll say that a "triple" in a string is a char appearing three times in a row.
// Return the number of triples in the given string. The triples may overlap.

// count_triple("abcXXXabc") → 1
// count_triple("xxxabyyyycd") → 3
// count_triple("a") → 0

fn count_triple(s: &str) -> u32 {
    if s.len() < 3 {
        0
    } else {
        let head = s[..3].as_bytes();
        if head[0] == head[1] && head[1] == head[2] {
            1 + count_triple(&s[1..])
        } else {
            0 + count_triple(&s[1..])
        }
    }
}

// String-3 > mirrorEnds
// https://codingbat.com/prob/p139411

// Given a string, look for a mirror image (backwards) string at both the beginning and end of the given string.
// In other words, zero or more characters at the very begining of the given string,
// and at the very end of the string in reverse order (possibly overlapping).
// For example, the string "abXYZba" has the mirror end "ab".

// mirror_ends("abXYZba") → "ab"
// mirror_ends("abca") → "a"
// mirror_ends("aba") → "aba"

fn mirror_ends(s: &str) -> String {
    for mirlen in (1..=s.len()).rev() {
        let reverse: String = s.chars().rev().collect();
        if s[..mirlen] == reverse[..mirlen] {
            return s[..mirlen].to_string()
        }
    }
    "".to_string()
}

// String-3 > notReplace
// https://codingbat.com/prob/p154137

// Given a string, return a string where every appearance of the lowercase word "is" has been replaced with "is not".
// The word "is" should not be immediately preceeded or followed by a letter
//  -- so for example the "is" in "this" does not count. (Note: Character.isLetter(char) tests if a char is a letter.)

// not_replace("is test") → "is not test"
// not_replace("is-is") → "is not-is not"
// not_replace("This is right") → "This is not right"

fn not_replace(s: &str) -> String {

    fn helper(s: &str, previous_char: &str) -> String {
        let slen = s.len();
        if slen == 2 && s == "is" && previous_char == "not letter" {
            "is not".to_string()
        } else if slen < 3 {
                s.to_string()
        } else {
            let head = s[..3].as_bytes();
            if &s[..2] == "is" && !head[2].is_ascii_alphabetic() && previous_char == "not letter" {
                format!("{}{}", "is not", helper(&s[2..], "not letter"))
            } else {
                let previous_char = if head[0].is_ascii_alphabetic() {"letter"} else {"not letter"};
                format!("{}{}", &s[..1], helper(&s[1..], previous_char))
            }
        }
    }
    helper(s, "not letter")
}

// String-3 > equalIsNot
// https://codingbat.com/prob/p141736

// Given a string, return true if the number of appearances of "is" anywhere in the string
// is equal to the number of appearances of "not" anywhere in the string (case sensitive).

// equal_is_not("This is not") → false
// equal_is_not("This is notnot") → true
// equal_is_not("noisxxnotyynotxisi") → true

fn equal_is_not(s: &str) -> bool {
    let slen = s.len();
    let mut is_count = 0;
    for index in 0..=slen-2 {
        if &s[index..index+2] == "is" {
            is_count += 1
        }
    }
    let mut not_count = 0;
    for index in 0..=slen-3 {
        if &s[index..index+3] == "not" {
            not_count += 1
        }
    }
    is_count == not_count
}

// String-3 > sumDigits
// https://codingbat.com/prob/p197890

// Given a string, return the sum of the digits 0-9 that appear in the string, ignoring all other characters.
// Return 0 if there are no digits in the string.

// sum_digits("aa1bc2d3") → 6
// sum_digits("aa11b33") → 8
// sum_digits("Chocolate") → 0

fn sum_digits(s: &str) -> u32 {
    let mut sum = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            sum += c.to_digit(10).unwrap();
        }
    }
    sum
}

// String-3 > maxBlock
// https://codingbat.com/prob/p179479

// Given a string, return the length of the largest "block" in the string.
// A block is a run of adjacent chars that are the same.

// max_block("hoopla") → 2
// max_block("abbCCCddBBBxx") → 3
// max_block("") → 0

fn max_block(s: &str) -> u32 {
    let mut previous_char = None;
    let mut count = 0;
    let mut max_count = 0;
    for c in s.chars() {
        match previous_char {
            Some(p) => if c == p {
                count += 1;
                if count > max_count {
                    max_count = count;
                }
            } else {
                // previous_char exists, but doesn't match
                count = 1;
                previous_char = Some(c);
            },
            None => previous_char = Some(c)
        }
    }
    max_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_yz_test() {
        assert_eq!(count_yz("fez day"), 2);
        assert_eq!(count_yz("day fez"), 2);
        assert_eq!(count_yz("day fyyyz"), 2);
        assert_eq!(count_yz("dax fyyyz"), 1);
        assert_eq!(count_yz("dax fyyya"), 0);
    }

    #[test]
    fn g_happy_test() {
        assert_eq!(g_happy("xxggxx"), true);
        assert_eq!(g_happy("xxgxx"), false);
        assert_eq!(g_happy("xxggyygxx"), false);
        assert_eq!(g_happy("xxgggyygxx"), false);
        assert_eq!(g_happy("xxggxxg"), false); // fail!
    }

    #[test]
    fn same_ends_test() {
        assert_eq!(same_ends("abXYab"), "ab");
        assert_eq!(same_ends("xx"), "x");
        assert_eq!(same_ends("xxx"), "x");
        assert_eq!(same_ends("mikemike"), "mike");
        assert_eq!(same_ends("mike"), "");
    }

    #[test]
    fn sum_numbers_test() {
        assert_eq!(sum_numbers("abc123xyz"), 123);
        assert_eq!(sum_numbers("aa11b33"), 44);
        assert_eq!(sum_numbers("7 11"), 18);
    }

    #[test]
    fn without_string_test() {
        assert_eq!(without_string("Hello there", "llo"), "He there");
        assert_eq!(without_string("Hello there", "e"), "Hllo thr");
        assert_eq!(without_string("Hello there", "x"), "Hello there");
        assert_eq!(without_string("HELLO THERE", "llo"), "HE THERE");
    }

    #[test]
    fn count_triple_test() {
        assert_eq!(count_triple("abcXXXabc"), 1);
        assert_eq!(count_triple("xxxabyyyycd"), 3);
        assert_eq!(count_triple("a"), 0);
    }

    #[test]
    fn mirror_ends_test() {
        assert_eq!(mirror_ends("abXYZba"), "ab");
        assert_eq!(mirror_ends("abca"), "a");
        assert_eq!(mirror_ends("aba"), "aba");
    }

    #[test]
    fn not_replace_test() {
        assert_eq!(not_replace("is test"), "is not test");
        assert_eq!(not_replace("is-is"), "is not-is not");
        assert_eq!(not_replace("This is right"), "This is not right");
    }

    #[test]
    fn equal_is_not_test() {
        assert_eq!(equal_is_not("This is not"), false);
        assert_eq!(equal_is_not("This is notnot"), true);
        assert_eq!(equal_is_not("noisxxnotyynotxisi"), true);
    }

    #[test]
    fn sum_digits_test() {
        assert_eq!(sum_digits("aa1bc2d3"), 6);
        assert_eq!(sum_digits("aa11b33"), 8);
        assert_eq!(sum_digits("Chocolate"), 0);
    }

    #[test]
    fn max_block_test() {
        assert_eq!(max_block("hoopla"), 2);
        assert_eq!(max_block("abbCCCddBBBxx"), 3);
        assert_eq!(max_block(""), 0);
    }
}
