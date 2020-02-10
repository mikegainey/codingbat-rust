// String-2 > doubleChar
// https://codingbat.com/prob/p165312

// Given a string, return a string where for every char in the original, there are two chars.

// double_char("The") → "TThhee"
// double_char("AAbb") → "AAAAbbbb"
// double_char("Hi-There") → "HHii--TThheerree"

fn double_char(s: &str) -> String {
    s.chars()
        .map(|c| format!("{0}{0}", c))
        .collect()
}

// String-2 > countCode
// https://codingbat.com/prob/p123614

// Return the number of times that the string "code" appears anywhere in the given string,
// except we'll accept any letter for the 'd', so "cope" and "cooe" count.

// count_code("aaacodebbb") → 1
// count_code("codexxcode") → 2
// count_code("cozexxcope") → 2

fn count_code1(s: &str) -> u32 {
    if s.len() < 4 {
        0
    } else {
        let head = &s[..4].as_bytes(); // head is type [u8]
        if &head[..2] == b"co" && head[3] == b'e' { // b"co" is type &[i8; 2]
            1 + count_code1(&s[4..])
        } else {
            0 + count_code1(&s[1..])
        }
    }
}

fn count_code2(s: &str) -> u32 {
    use regex::Regex;
    let re = Regex::new(r"^co.e").unwrap();

    let mut count = 0;
    for x in 0..=s.len()-4 {
        if re.is_match(&s[x..x+4]) {
            count += 1;
        }
    }
    count
}

// String-2 > bobThere
// https://codingbat.com/prob/p175762

// Return true if the given string contains a "bob" string, but where the middle 'o' char can be any char.

// bob_there("abcbob") → true
// bob_there("b9b") → true
// bob_there("bac") → false

fn bob_there(s: &str) -> bool {
    use regex::Regex;
    let re = Regex::new(r"^b.b").unwrap();

    for x in 0..=s.len()-3 {
        if re.is_match(&s[x..x+3]) {
            return true;
        }
    }
    false
}

// String-2 > repeatEnd
// https://codingbat.com/prob/p152339

// Given a string and an int n, return a string made of n repetitions of the last n characters of the string.
// You may assume that n is between 0 and the length of the string, inclusive.

// repeat_end("Hello", 3) → "llollollo"
// repeat_end("Hello", 2) → "lolo"
// repeat_end("Hello", 1) → "o"

fn repeat_end(s: &str, n: usize) -> String {
    let len = s.len();
    s[len-n..].repeat(n)
}

// String-2 > prefixAgain
// https://codingbat.com/prob/p136417

// Given a string, consider the prefix string made of the first N chars of the string.
// Does that prefix string appear somewhere else in the string?
// Assume that the string is not empty and that N is in the range 1..str.length().

// prefix_again("abXYabc", 1) → true
// prefix_again("abXYabc", 2) → true
// prefix_again("abXYabc", 3) → false

fn prefix_again(s: &str, n: usize) -> bool {
    let sub = &s[..n];
    let sub_len = sub.len();
    for x in 1..= s.len() - sub_len {
        if &s[x..(x + sub_len)] == sub {
            return true;
        }
    }
    false
}

// String-2 > sameStarChar
// https://codingbat.com/prob/p194491

// Returns true if for every '*' (star) in the string, if there are chars both immediately before and after the star,
// they are the same.

// same_star_char("xy*yzz") → true
// same_star_char("xy*zzz") → false
// same_star_char("*xa*az") → true

fn same_star_char(s: &str) -> bool {
    for x in 0..=s.len()-3 {
        let group = &s[x..x+3].as_bytes();
        if group[1] == b'*' && group[0] == group[2] {
            return true;
        }
    }
    false
}

// String-2 > starOut
// https://codingbat.com/prob/p139564

// Return a version of the given string,
// where for every star (*) in the string the star and the chars immediately to its left and right are gone.
// So "ab*cd" yields "ad" and "ab**cd" also yields "ad".

// star_out("ab*cd") → "ad"
// star_out("ab**cd") → "ad"
// star_out("sm*eilly") → "silly"

// with a for loop, pushing to an output variable
fn star_out1(s: &str) -> String {
    let mut output = String::new();
    let len = s.len();

    // if the first two chars are letters, push the first
    if s[..2].chars().all(|c| c.is_ascii_alphabetic()) {
        output.push(s.chars().next().unwrap());
    }

    // if a group of three chars are letters, push the middle
    for x in 0..=len-3 {
        let group = s[x..x+3].as_bytes();
        if group.iter().all(|&c| (c as char).is_ascii_alphabetic()) {
            output.push(group[1] as char);
        }
    }

    //if the last two chars are letters, push the last
    if s[len-2..].chars().all(|c| c.is_ascii_alphabetic()) {
        output.push(s.chars().last().unwrap());
    }

    output
}

// with recursion
fn star_out2(s: &str) -> String {
    if s.len() < 2 {
        format!("{}", s)
    } else {
        let head = &s[..2].as_bytes();
        if !head.contains(&b'*') {
            format!("{}{}", &s[..1], star_out2(&s[1..]))
        } else if head[1] == b'*' {
            format!("{}", star_out2(&s[1..]))
        } else if head[0] == b'*' {
            format!("{}", star_out2(&s[2..]))
        } else {
            format!("{}", star_out2(&s[1..]))
        }
    }
}

// String-2 > countHi
// https://codingbat.com/prob/p147448

// Return the number of times that the string "hi" appears anywhere in the given string.

// count_hi("abc hi ho") → 1
// count_hi("ABChi hi") → 2
// count_hi("hihi") → 2

fn count_hi(s: &str) -> u32 {
    let len = s.len();
    let mut count = 0;
    for x in 0..=len-2 {
        if &s[x..x+2] == "hi" {
            count += 1;
        }
    }
    count
}

// String-2 > endOther
// https://codingbat.com/prob/p126880

// Given two strings, return true if either of the strings appears at the very end of the other string,
// ignoring upper/lower case differences (in other words, the computation should not be "case sensitive").

// end_other("Hiabc", "abc") → true
// end_other("AbC", "HiaBc") → true
// end_other("abc", "abXabc") → true

fn end_other(a: &str, b: &str) -> bool {
    let (a_len, b_len) = (a.len(), b.len());
    let min_len = std::cmp::min(a_len, b_len);
    a[a_len - min_len ..].to_lowercase() == b[b_len - min_len ..].to_lowercase()
}

// String-2 > xyBalance
// https://codingbat.com/prob/p134250

// We'll say that a String is xy-balanced if for all the 'x' chars in the string,
// there exists a 'y' char somewhere later in the string. So "xxy" is balanced, but "xyx" is not.
// One 'y' can balance multiple 'x's. Return true if the given string is xy-balanced.

// xy_balance("aaxbby") → true
// xy_balance("aaxbb") → false
// xy_balance("yaaxbb") → false

fn xy_balance(s: &str) -> bool {
    for c in s.chars().rev() {
        if c == 'y' {
            return true;
        } else if c == 'x' {
            return false;
        }
    }
    true // if there are no x's or y's, consider the string balanced
}

// String-2 > repeatFront
// https://codingbat.com/prob/p128796

// Given a string and an int n, return a string made of the first n characters of the string,
// followed by the first n-1 characters of the string, and so on.
// You may assume that n is between 0 and the length of the string, inclusive (i.e. n >= 0 and n <= str.length()).

// repeat_front("Chocolate", 4) → "ChocChoChC"
// repeat_front("Chocolate", 3) → "ChoChC"
// repeat_front("Ice Cream", 2) → "IcI"

fn repeat_front1(s: &str, n: usize) -> String {
    let mut output = String::new();
    for len in (1..=n).rev() {
        output.push_str(&s[..len]);
    }
    output
}

// String-2 > xyzMiddle
// https://codingbat.com/prob/p159772

// Given a string, does "xyz" appear in the middle of the string?
// To define middle, we'll say that the number of chars to the left and right of the "xyz" must differ by at most one.
// This problem is harder than it looks.

// xyz_middle("AAxyzBB") → true
// xyz_middle("AxyzBB") → true
// xyz_middle("AxyzBBB") → false

fn xyz_middle(s: &str) -> bool {
    let len = s.len();
    let mid = len / 2;
    if len % 2 == 1 {
        &s[mid-1..mid+2] == "xyz"
    } else {
        &s[mid-2..mid+1] == "xyz" || &s[mid-1..mid+2] == "xyz"
    }
}

// String-2 > oneTwo
// https://codingbat.com/prob/p122943

// Given a string, compute a new string by moving the first char to come after the next two chars,
// so "abc" yields "bca". Repeat this process for each subsequent group of 3 chars, so "abcdef" yields "bcaefd".
// Ignore any group of fewer than 3 chars at the end.

// one_two("abc") → "bca"
// one_two("tca") → "cat"
// one_two("tcagdo") → "catdog"

fn one_two(s: &str) -> String {
    if s.len() < 3 {
        "".to_string()
    } else {
        let abc = s[..3].as_bytes();
        let bca = [abc[1], abc[2], abc[0]];
        format!("{}{}", std::str::from_utf8(&bca).unwrap(), one_two(&s[3..]))
    }
}

// String-2 > plusOut
// https://codingbat.com/prob/p170829

// Given a string and a non-empty word string, return a version of the original String
// where all chars have been replaced by pluses ("+"),
// except for appearances of the word string which are preserved unchanged.

// plus_out("12xy34", "xy") → "++xy++"
// plus_out("12xy34", "1") → "1+++++"
// plus_out("12xy34xyabcxy", "xy") → "++xy++xy+++xy"

fn plus_out(s: &str, word: &str) -> String {
    let (slen, wlen) = (s.len(), word.len());
    if slen < wlen {
        "+".repeat(slen)
    } else {
        if &s[..wlen] == word {
            format!("{}{}", &s[..wlen], plus_out(&s[wlen..], word))
        } else {
            format!("{}{}", "+", plus_out(&s[1..], word))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_char_test() {
        assert_eq!(double_char("The"), "TThhee");
        assert_eq!(double_char("AAbb"), "AAAAbbbb");
        assert_eq!(double_char("Hi-There"), "HHii--TThheerree");
    }

    #[test]
    fn count_code1_test() {
        assert_eq!(count_code1("aaacodebbb"), 1);
        assert_eq!(count_code1("codexxcode"), 2);
        assert_eq!(count_code1("cozexxcope"), 2);
    }

    #[test]
    fn count_code2_test() {
        assert_eq!(count_code2("aaacodebbb"), 1);
        assert_eq!(count_code2("codexxcode"), 2);
        assert_eq!(count_code2("cozexxcope"), 2);
    }

    #[test]
    fn bob_there_test() {
        assert_eq!(bob_there("abcbob"), true);
        assert_eq!(bob_there("b9b"), true);
        assert_eq!(bob_there("bac"), false);
    }

    #[test]
    fn repeat_end_test() {
        assert_eq!(repeat_end("Hello", 3), "llollollo");
        assert_eq!(repeat_end("Hello", 2), "lolo");
        assert_eq!(repeat_end("Hello", 1), "o");
    }

    #[test]
    fn prefix_again_test() {
        assert_eq!(prefix_again("abXYabc", 1), true);
        assert_eq!(prefix_again("abXYabc", 2), true);
        assert_eq!(prefix_again("abXYabc", 3), false);
    }

    #[test]
    fn same_star_char_test() {
        assert_eq!(same_star_char("xy*yzz"), true);
        assert_eq!(same_star_char("xy*zzz"), false);
        assert_eq!(same_star_char("*xa*az"), true);
    }

    #[test]
    fn star_out1_test() {
        assert_eq!(star_out1("ab*cd"), "ad");
        assert_eq!(star_out1("ab**cd"), "ad");
        assert_eq!(star_out1("sm*eilly"), "silly");
    }

    #[test]
    fn star_out2_test() {
        assert_eq!(star_out2("ab*cd"), "ad");
        assert_eq!(star_out2("ab**cd"), "ad");
        assert_eq!(star_out2("sm*eilly"), "silly");
    }

    #[test]
    fn count_hi_test() {
        assert_eq!(count_hi("abc hi ho"), 1);
        assert_eq!(count_hi("ABChi hi"), 2);
        assert_eq!(count_hi("hihi"), 2);
    }

    #[test]
    fn end_other_test() {
        assert_eq!(end_other("Hiabc", "abc"), true);
        assert_eq!(end_other("AbC", "HiaBc"), true);
        assert_eq!(end_other("abc", "abXabc"), true);
    }

    #[test]
    fn xy_balance_test() {
        assert_eq!(xy_balance("aaxbby"), true);
        assert_eq!(xy_balance("aaxbb"), false);
        assert_eq!(xy_balance("yaaxbb"), false);
    }

    #[test]
    fn repeat_front1_test() {
        assert_eq!(repeat_front1("Chocolate", 4), "ChocChoChC");
        assert_eq!(repeat_front1("Chocolate", 3), "ChoChC");
        assert_eq!(repeat_front1("Ice Cream", 2), "IcI");
    }

    #[test]
    fn xyz_middle_test() {
        assert_eq!(xyz_middle("AAxyzBB"), true);
        assert_eq!(xyz_middle("AxyzBB"), true);
        assert_eq!(xyz_middle("AxyzBBB"), false);
    }

    #[test]
    fn one_two_test() {
        assert_eq!(one_two("abc"), "bca");
        assert_eq!(one_two("tca"), "cat");
        assert_eq!(one_two("tcagdo"), "catdog");
    }

    #[test]
    fn plus_out_test() {
        assert_eq!(plus_out("12xy34", "xy"), "++xy++");
        assert_eq!(plus_out("12xy34", "1"), "1+++++");
        assert_eq!(plus_out("12xy34xyabcxy", "xy"), "++xy++xy+++xy");
    }
}
