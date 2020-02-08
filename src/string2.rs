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

fn star_out(s: &str) -> String {
    if s.len() < 2 {
        s.to_string()
    } else {
        let head = &s[..2].as_bytes();
        if !head.contains(&b'*') {
            format!("{}{}", &s[..1], star_out(&s[1..]))
        } else if head[1] == b'*' {
            format!("{}", star_out(&s[1..]))
        } else if head[0] == b'*' {
            format!("{}", star_out(&s[2..]))
        } else {
            format!("{}", star_out(&s[1..]))
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
    fn star_out_test() {
        assert_eq!(star_out("ab*cd"), "ad");
        assert_eq!(star_out("ab**cd"), "ad");
        assert_eq!(star_out("sm*eilly"), "silly");
    }
}
