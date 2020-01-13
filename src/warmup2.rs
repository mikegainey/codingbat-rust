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

    // with a for loop
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

    // with a recursive helper function
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

fn array123a(a: &[i32]) -> bool {
    let len = a.len();
    for x in 0..=len-3 {
        if &a[x..x+3] == &[1,2,3] {
            return true
        }
    }
    false
}

fn array123b(a: &[i32]) -> bool {
    if a.len() < 3 {
        return false
    } else {
        return (&a[0..3] == &[1,2,3]) || array123b(&a[1..])
    }
}

fn array123c(a: &[i32]) -> bool {
    let len = a.len();
    let count123 = (0..=len-3)
        .map(|x| (a[x], a[x+1], a[x+2]))
        .filter(|&x| x == (1,2,3))
        .count();

    count123 >= 1
}

// Warmup-2 > altPairs
// https://codingbat.com/prob/p121596

// Given a string, return a string made of the chars at indexes 0,1, 4,5, 8,9 ... so "kittens" yields "kien".

// alt_pairs("kitten") → "kien"
// alt_pairs("Chocolate") → "Chole"
// alt_pairs("CodingHorror") → "Congrr"

// with recursion
fn alt_pairs1(s: &str) -> String {
    if s.len() <= 2 {
        s.to_string()
    } else if s.len() <= 4 {
        format!("{}", &s[..2])
    } else {
        format!("{}{}", &s[..2], alt_pairs1(&s[4..]))
    }
}

// with iteration, pushing to accumulate an output variable
fn alt_pairs2(s: &str) -> String {
    let mut output = String::new();
    let mut mod4x;
    for (x, c) in s.chars().enumerate() {
        mod4x = x % 4;
        if mod4x == 0 || mod4x == 1 {
            output.push(c);
        }
    }
    output
}

// with iterator adaptors
fn alt_pairs3(s: &str) -> String {
    s.chars()
        .enumerate()
        .filter(|(x,_)| x % 4 == 0 || x % 4 == 1)
        .map(|(_,c)| c)
        .collect::<String>()
}

// Warmup-2 > noTriples
// https://codingbat.com/prob/p170221

// Given an array of ints, we'll say that a triple is a value appearing 3 times in a row in the array.
// Return true if the array does not contain any triples.

// no_triples([1, 1, 2, 2, 1]) → true
// no_triples([1, 1, 2, 2, 2, 1]) → false
// no_triples([1, 1, 1, 2, 2, 2, 1]) → false

fn no_triples1(a: &[i32]) -> bool {
    let len = a.len();
    for x in 0..=len-3 {
        if a[x] == a[x+1] && a[x] == a[x+2] {
            return false
        }
    }
    true
}

fn no_triples2(a: &[i32]) -> bool {
    if a.len() < 3 {
        true
    } else {
        let no_triple = !(a[0] == a[1] && a[0] == a[2]);
        no_triple && no_triples2(&a[1..])
    }
}

// Warmup-2 > frontTimes
// https://codingbat.com/prob/p101475

// Given a string and a non-negative int n, we'll say that the front of the string is the first 3 chars,
// or whatever is there if the string is less than length 3.
// Return n copies of the front;

// front_times("Chocolate", 2) → "ChoCho"
// front_times("Chocolate", 3) → "ChoChoCho"
// front_times("Abc", 3) → "AbcAbcAbc"

fn front_times(s: &str, n: usize) -> String {
    use std::cmp;

    let lastx = cmp::min(3, s.len());
    let front = &s[..lastx];

    front.repeat(n)
}

// Warmup-2 > stringBits
// https://codingbat.com/prob/p165666

// Given a string, return a new string made of every other char starting with the first, so "Hello" yields "Hlo".

// string_bits("Hello") → "Hlo"
// string_bits("Hi") → "H"
// string_bits("Heeololeo") → "Hello"

fn string_bits(s: &str) -> String {
    s.chars()
        .step_by(2)
        .collect::<String>()
}

// Warmup-2 > arrayCount9
// https://codingbat.com/prob/p184031

// Given an array of ints, return the number of 9's in the array.

// array_count9([1, 2, 9]) → 1
// array_count9([1, 9, 9]) → 2
// array_count9([1, 9, 9, 3, 9]) → 3

fn array_count9a(a: &[i32]) -> usize {
    a.iter()
        .filter(|&x| x == &9)
        .count()
}

fn array_count9b(a: &[i32]) -> usize {
    let mut count = 0;
    for x in a.iter() {
        if x == &9 {
            count += 1;
        }
    }
    count
}

// Warmup-2 > stringMatch
// https://codingbat.com/prob/p198640

// Given 2 strings, a and b, return the number of the positions where they contain the same length 2 substring.
// So "xxcaazz" and "xxbaaz" yields 3, since the "xx", "aa", and "az" substrings appear in the same place in both strings.

// string_match("xxcaazz", "xxbaaz") → 3
// string_match("abc", "abc") → 2
// string_match("abc", "axc") → 0

fn string_match(a: &str, b: &str) -> u32 {
    use std::cmp;

    let mut count = 0;
    let len = cmp::min(a.len(), b.len());
    for x in 0..=len-2 {
        if &a[x..=x+1] == &b[x..=x+1] {
            count += 1;
        }
    }
    count
}

// Warmup-2 > stringYak
// https://codingbat.com/prob/p126212

// Suppose the string "yak" is unlucky. Given a string, return a version where all the "yak" are removed,
// but the "a" can be any char. The "yak" strings will not overlap.

// string_yak("yakpak") → "pak"
// string_yak("pakyak") → "pak"
// string_yak("yak123ya") → "123ya"

fn string_yak1(s: &str) -> String {

    fn is_yak(s: &str) -> bool {
        let s_bytes = s.as_bytes();
        s_bytes[0] == b'y' && s_bytes[2] == b'k'
    }

    if s.len() < 3 {
        s.to_string()
    } else {
        if is_yak(&s[0..3]) {
            format!("{}", string_yak1(&s[3..]))
        } else {
            format!("{}{}", &s[0..1], string_yak1(&s[1..]))
        }
    }
}

fn string_yak2(s: &str) -> String {
    use regex::Regex;
    let yak_re = Regex::new(r"^y.k").unwrap();

    if s.len() < 3 {
        s.to_string()
    } else if yak_re.is_match(&s[..3]) {
        string_yak2(&s[3..])
    } else {
        format!("{}{}", &s[..1], string_yak2(&s[1..]))
    }
}

// Warmup-2 > has271
// https://codingbat.com/prob/p167430

// Given an array of ints, return true if it contains a 2, 7, 1 pattern:
// a value, followed by the value plus 5, followed by the value minus 1.
// Additionally the 271 counts even if the "1" differs by 2 or less from the correct value.

// has271([1, 2, 7, 1]) → true
// has271([1, 2, 8, 1]) → false
// has271([2, 7, 1]) → true

fn has271(a: &[i32]) -> bool {
    if a.len() < 3 {
        false
    } else {
        let ab = (a[1] - a[0]) == 5;
        let ac1 = a[2] - a[0]; // allowed to be -1 (+/- 2)
        let ac2 = ac1 >= -3 && ac1 <= 1;
        if ab && ac2 {
            true
        } else {
            has271(&a[1..])
        }
    }
}

// Warmup-2 > countXX
// https://codingbat.com/prob/p194667

// Count the number of "xx" in the given string. We'll say that overlapping is allowed, so "xxx" contains 2 "xx".

// count_xx("abcxx") → 1
// count_xx("xxx") → 2
// count_xx("xxxx") → 3

fn count_xx(s: &str) -> u32 {
    if s.len() < 2 {
        0
    } else if &s[..2] == "xx" {
        1 + count_xx(&s[1..])
    } else {
        0 + count_xx(&s[1..])
    }
}

// Warmup-2 > stringSplosion
// https://codingbat.com/prob/p117334

// Given a non-empty string like "Code" return a string like "CCoCodCode".

// string_splosion("Code") → "CCoCodCode"
// string_splosion("abc") → "aababc"
// string_splosion("ab") → "aab"

fn string_splosion(s: &str) -> String {
    let mut output = String::new();
    for len in 1..=s.len() {
        output.push_str(&s[..len]);
    }
    output
}

// Warmup-2 > arrayFront9
// https://codingbat.com/prob/p186031

// Given an array of ints, return true if one of the first 4 elements in the array is a 9.
// The array length may be less than 4.

// array_front9([1, 2, 9, 3, 4]) → true
// array_front9([1, 2, 3, 4, 9]) → false
// array_front9([1, 2, 3, 4, 5]) → false

fn array_front9(a: &[i32]) -> bool {
    use std::cmp;

    let lastx = cmp::min(a.len(), 4);
    let a = &a[..lastx];

    let count9 = a.iter()
        .filter(|&x| x == &9)
        .count();
    count9 >= 1
}

// Warmup-2 > stringX
// https://codingbat.com/prob/p171260

// Given a string, return a version where all the "x" have been removed.
// Except an "x" at the very start or end should not be removed.

// string_x("xxHxix") → "xHix"
// string_x("abxxxcd") → "abcd"
// string_x("xabxxxcdx") → "xabcdx"

fn string_x(s: &str) -> String {
    let len = s.len();
    let first = &s[..1];
    let last = &s[len-1..];
    let middle = &s[1..len-1].chars()
        .filter(|&c| c != 'x')
        .collect::<String>();
    format!("{}{}{}", first, middle, last)
}

// Warmup-2 > array667
// https://codingbat.com/prob/p110019

// Given an array of ints, return the number of times that two 6's are next to each other in the array.
// Also count instances where the second "6" is actually a 7.

// array667([6, 6, 2]) → 1
// array667([6, 6, 2, 6]) → 1
// array667([6, 7, 2, 6]) → 1

fn array667(a: &[i32]) -> u32 {
    if a.len() < 2 {
        0
    } else if a[0] == 6 && (a[1] == 6 || a[1] == 7) {
        1 + array667(&a[1..])
    } else {
        0 + array667(&a[1..])
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
        assert_eq!(array123a(&[1, 1, 2, 3, 1]), true);
        assert_eq!(array123a(&[1, 1, 2, 4, 1]), false);
        assert_eq!(array123a(&[1, 1, 2, 1, 2, 3]), true);
        assert_eq!(array123a(&[1, 2, 3, 1, 2, 4]), true);
        assert_eq!(array123b(&[1, 1, 2, 3, 1]), true);
        assert_eq!(array123b(&[1, 1, 2, 4, 1]), false);
        assert_eq!(array123b(&[1, 1, 2, 1, 2, 3]), true);
        assert_eq!(array123b(&[1, 2, 3, 1, 2, 4]), true);
        assert_eq!(array123c(&[1, 1, 2, 3, 1]), true);
        assert_eq!(array123c(&[1, 1, 2, 4, 1]), false);
        assert_eq!(array123c(&[1, 1, 2, 1, 2, 3]), true);
        assert_eq!(array123c(&[1, 2, 3, 1, 2, 4]), true);
    }

    #[test]
    fn alt_pairs_test() {
        assert_eq!(alt_pairs1("kitten"), "kien");
        assert_eq!(alt_pairs1("Chocolate"), "Chole");
        assert_eq!(alt_pairs1("CodingHorror"), "Congrr");
        assert_eq!(alt_pairs2("kitten"), "kien");
        assert_eq!(alt_pairs2("Chocolate"), "Chole");
        assert_eq!(alt_pairs2("CodingHorror"), "Congrr");
        assert_eq!(alt_pairs3("kitten"), "kien");
        assert_eq!(alt_pairs3("Chocolate"), "Chole");
        assert_eq!(alt_pairs3("CodingHorror"), "Congrr");
    }

    #[test]
    fn no_triples_test() {
        assert_eq!(no_triples1(&[1, 1, 2, 2, 1]), true);
        assert_eq!(no_triples1(&[1, 1, 2, 2, 2, 1]), false);
        assert_eq!(no_triples1(&[1, 1, 1, 2, 2, 2, 1]), false);
        assert_eq!(no_triples2(&[1, 1, 2, 2, 1]), true);
        assert_eq!(no_triples2(&[1, 1, 2, 2, 2, 1]), false);
        assert_eq!(no_triples2(&[1, 1, 1, 2, 2, 2, 1]), false);
    }

    #[test]
    fn front_times_test() {
        assert_eq!(front_times("Chocolate", 2), "ChoCho");
        assert_eq!(front_times("Chocolate", 3), "ChoChoCho");
        assert_eq!(front_times("Abc", 3), "AbcAbcAbc");
    }

    #[test]
    fn string_bits_test() {
        assert_eq!(string_bits("Hello"), "Hlo");
        assert_eq!(string_bits("Hi"), "H");
        assert_eq!(string_bits("Heeololeo"), "Hello");
    }

    #[test]
    fn array_count9_test() {
        assert_eq!(array_count9a(&[1, 2, 9]), 1);
        assert_eq!(array_count9a(&[1, 9, 9]), 2);
        assert_eq!(array_count9a(&[1, 9, 9, 3, 9]), 3);
        assert_eq!(array_count9b(&[1, 2, 9]), 1);
        assert_eq!(array_count9b(&[1, 9, 9]), 2);
        assert_eq!(array_count9b(&[1, 9, 9, 3, 9]), 3);
    }

    #[test]
    fn string_match_test() {
        assert_eq!(string_match("xxcaazz", "xxbaaz"), 3);
        assert_eq!(string_match("abc", "abc"), 2);
        assert_eq!(string_match("abc", "axc"), 0);
    }

    #[test]
    fn string_yak_test() {
        assert_eq!(string_yak1("yakpak"), "pak");
        assert_eq!(string_yak1("pakyak"), "pak");
        assert_eq!(string_yak1("yak123ya"), "123ya");
        assert_eq!(string_yak1("ayakpak"), "apak"); // yak starting at the 2nd character
        assert_eq!(string_yak2("yakpak"), "pak");
        assert_eq!(string_yak2("pakyak"), "pak");
        assert_eq!(string_yak2("yak123ya"), "123ya");
        assert_eq!(string_yak2("ayakpak"), "apak"); // yak starting at the 2nd character
    }

    #[test]
    fn has271_test() {
        assert_eq!(has271(&[1, 2, 7, 1]), true);
        assert_eq!(has271(&[1, 2, 8, 1]), false);
        assert_eq!(has271(&[2, 7, 1]), true);
        assert_eq!(has271(&[3, 8, 2]), true);
        assert_eq!(has271(&[3, 8, 3]), true);
        assert_eq!(has271(&[3, 8, 4]), true);
        assert_eq!(has271(&[3, 8, 5]), false);
        assert_eq!(has271(&[3, 8, 1]), true);
        assert_eq!(has271(&[3, 8, 0]), true);
        assert_eq!(has271(&[4, 9, 0]), false);
    }

    #[test]
    fn count_xx_test() {
        assert_eq!(count_xx("abcxx"), 1);
        assert_eq!(count_xx("xxx"), 2);
        assert_eq!(count_xx("xxxx"), 3);
    }

    #[test]
    fn string_splosion_test() {
        assert_eq!(string_splosion("Code"), "CCoCodCode");
        assert_eq!(string_splosion("abc"), "aababc");
        assert_eq!(string_splosion("ab"), "aab");
    }

    #[test]
    fn array_front9_test() {
        assert_eq!(array_front9(&[1, 2, 9, 3, 4]), true);
        assert_eq!(array_front9(&[1, 2, 3, 4, 9]), false);
        assert_eq!(array_front9(&[1, 2, 3, 4, 5]), false);
    }

    #[test]
    fn string_x_test() {
        assert_eq!(string_x("xxHxix"), "xHix");
        assert_eq!(string_x("abxxxcd"), "abcd");
        assert_eq!(string_x("xabxxxcdx"), "xabcdx");
    }

    #[test]
    fn array667_test() {
        assert_eq!(array667(&[6, 6, 2]), 1);
        assert_eq!(array667(&[6, 6, 2, 6]), 1);
        assert_eq!(array667(&[6, 7, 2, 6]), 1);
    }
}
