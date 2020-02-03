// String-1 > helloName
// https://codingbat.com/prob/p171896

// Given a string name, e.g. "Bob", return a greeting of the form "Hello Bob!".

// hello_name("Bob") → "Hello Bob!"
// hello_name("Alice") → "Hello Alice!"
// hello_name("X") → "Hello X!"

fn hello_name(s: &str) -> String {
    format!("Hello {}!", s)
}

// String-1 > makeOutWord
// https://codingbat.com/prob/p184030

// Given an "out" string length 4, such as "<<>>", and a word,
// return a new string where the word is in the middle of the out string, e.g. "<<word>>".

// make_out_word("<<>>", "Yay") → "<<Yay>>"
// make_out_word("<<>>", "WooHoo") → "<<WooHoo>>"
// make_out_word("[[]]", "word") → "[[word]]"

fn make_out_word(out: &str, word: &str) -> String {
    let open = &out[..2];
    let close = &out[2..];
    format!("{}{}{}", open, word, close)
}

// String-1 > firstHalf
// https://codingbat.com/prob/p172267

// Given a string of even length, return the first half. So the string "WooHoo" yields "Woo".

// first_half("WooHoo") → "Woo"
// first_half("HelloThere") → "Hello"
// first_half("abcdef") → "abc"

fn first_half(s: &str) -> &str {
    let len = s.len();
    &s[..len/2]
}

// String-1 > nonStart
// https://codingbat.com/prob/p143825

// Given 2 strings, return their concatenation, except omit the first char of each. The strings will be at least length 1.

// non_start("Hello", "There") → "ellohere"
// non_start("java", "code") → "avaode"
// non_start("shotl", "java") → "hotlava"

fn non_start(a: &str, b: &str) -> String {
    format!("{}{}", &a[1..], &b[1..])
}

// String-1 > theEnd
// https://codingbat.com/prob/p162477

// Given a string, return a string length 1 from its front,
// unless front is false, in which case return a string length 1 from its back.
// The string will be non-empty.

// the_end("Hello", true) → "H"
// the_end("Hello", false) → "o"
// the_end("oh", true) → "o"

fn the_end(s: &str, front: bool) -> &str {
    if front {
        &s[..1]
    } else {
        let end = s.len()-1;
        &s[end..]
    }
}

// String-1 > endsLy
// https://codingbat.com/prob/p103895

// Given a string, return true if it ends in "ly".

// ends_ly("oddly") → true
// ends_ly("y") → false
// ends_ly("oddy") → false

fn ends_ly(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }
    let end = s.len()-1;
    &s[end-1..] == "ly"
}

// String-1 > middleThree
// https://codingbat.com/prob/p115863

// Given a string of odd length, return the string length 3 from its middle,
// so "Candy" yields "and". The string length will be at least 3.

// middle_three("Candy") → "and"
// middle_three("and") → "and"
// middle_three("solving") → "lvi"

fn middle_three(s: &str) -> &str {
    let midx = s.len() / 2;
    &s[midx-1..=midx+1]
}

// String-1 > lastChars
// https://codingbat.com/prob/p138183

// Given 2 strings, a and b, return a new string made of the first char of a and the last char of b,
// so "yo" and "java" yields "ya". If either string is length 0, use '@' for its missing char.

// last_chars("last", "chars") → "ls"
// last_chars("yo", "java") → "ya"
// last_chars("hi", "") → "h@"

fn last_chars(a: &str, b: &str) -> String {
    let first = a.chars().next().unwrap_or('@');
    let last = b.chars().rev().next().unwrap_or('@');
    format!("{}{}", first, last)
}

// String-1 > seeColor
// https://codingbat.com/prob/p199216

// Given a string, if the string begins with "red" or "blue" return that color string, otherwise return the empty string.

// see_color("redxx") → "red"
// see_color("xxred") → ""
// see_color("blueTimes") → "blue"

fn see_color(s: &str) -> &str {
    if s.len() >= 3 && &s[..3] == "red" {
        &s[..3]
    } else if s.len() >= 4 && &s[..4] == "blue" {
        &s[..4]
    } else {
        ""
    }
}

// String-1 > extraFront
// https://codingbat.com/prob/p172063

// Given a string, return a new string made of 3 copies of the first 2 chars of the original string.
// The string may be any length. If there are fewer than 2 chars, use whatever is there.

// extra_front("Hello") → "HeHeHe"
// extra_front("ab") → "ababab"
// extra_front("H") → "HHH"

fn extra_front(s: &str) -> String {
    use std::cmp;
    let len = cmp::min(2, s.len());
    format!("{0}{0}{0}", &s[..len])
}

// String-1 > startWord
// https://codingbat.com/prob/p141494

// Given a string and a second "word" string,
// we'll say that the word matches the string if it appears at the front of the string,
// except its first char does not need to match exactly.
// On a match, return the front of the string, or otherwise return the empty string.
// So, so with the string "hippo" the word "hi" returns "hi" and "xip" returns "hip".
// The word will be at least length 1.

// start_word("hippo", "hi") → "hi"
// start_word("hippo", "xip") → "hip"
// start_word("hippo", "i") → "h"

fn start_word<'a>(string: &'a str, word: &str) -> &'a str {
    for (index, (s, w)) in string.chars().zip(word.chars()).enumerate() {
        if (s != w) && (index > 0) {
            return &string[..index];
        }
    }
    &string[..word.len()]
}

// String-1 > makeAbba
// https://codingbat.com/prob/p161056

// Given two strings, a and b, return the result of putting them together in the order abba,
// e.g. "Hi" and "Bye" returns "HiByeByeHi".

// make_abba("Hi", "Bye") → "HiByeByeHi"
// make_abba("Yo", "Alice") → "YoAliceAliceYo"
// make_abba("What", "Up") → "WhatUpUpWhat"

fn make_abba(a: &str, b: &str) -> String {
    format!("{0}{1}{1}{0}", a, b)
}

// String-1 > extraEnd
// https://codingbat.com/prob/p108853

// Given a string, return a new string made of 3 copies of the last 2 chars of the original string.
// The string length will be at least 2.

// extra_end("Hello") → "lololo"
// extra_end("ab") → "ababab"
// extra_end("Hi") → "HiHiHi"

fn extra_end(s: &str) -> String {
    let len = s.len();
    format!("{0}{0}{0}", &s[len-2..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_name_test() {
        assert_eq!(hello_name("Bob"), "Hello Bob!");
        assert_eq!(hello_name("Alice"), "Hello Alice!");
        assert_eq!(hello_name("X"), "Hello X!");
    }

    #[test]
    fn make_out_word_test() {
        assert_eq!(make_out_word("<<>>", "Yay"), "<<Yay>>");
        assert_eq!(make_out_word("<<>>", "WooHoo"), "<<WooHoo>>");
        assert_eq!(make_out_word("[[]]", "word"), "[[word]]");
    }

    #[test]
    fn first_half_test() {
        assert_eq!(first_half("WooHoo"), "Woo");
        assert_eq!(first_half("HelloThere"), "Hello");
        assert_eq!(first_half("abcdef"), "abc");
    }

    #[test]
    fn non_start_test() {
        assert_eq!(non_start("Hello", "There"), "ellohere");
        assert_eq!(non_start("java", "code"), "avaode");
        assert_eq!(non_start("shotl", "java"), "hotlava");
    }

    #[test]
    fn the_end_test() {
        assert_eq!(the_end("Hello", true), "H");
        assert_eq!(the_end("Hello", false), "o");
        assert_eq!(the_end("oh", true), "o");
    }

    #[test]
    fn ends_ly_test() {
        assert_eq!(ends_ly("oddly"), true);
        assert_eq!(ends_ly("y"), false);
        assert_eq!(ends_ly("oddy"), false);
    }

    #[test]
    fn middle_three_test() {
        assert_eq!(middle_three("Candy"), "and");
        assert_eq!(middle_three("and"), "and");
        assert_eq!(middle_three("solving"), "lvi");
    }

    #[test]
    fn last_chars_test() {
        assert_eq!(last_chars("last", "chars"), "ls");
        assert_eq!(last_chars("yo", "java"), "ya");
        assert_eq!(last_chars("hi", ""), "h@");
    }

    #[test]
    fn see_color_test() {
        assert_eq!(see_color("redxx"), "red");
        assert_eq!(see_color("xxred"), "");
        assert_eq!(see_color("blueTimes"), "blue");
    }

    #[test]
    fn extra_front_test() {
        assert_eq!(extra_front("Hello"), "HeHeHe");
        assert_eq!(extra_front("ab"), "ababab");
        assert_eq!(extra_front("H"), "HHH");
    }

    #[test]
    fn start_word_test() {
        assert_eq!(start_word("hippo", "hi"), "hi");
        assert_eq!(start_word("hippo", "xip"), "hip");
        assert_eq!(start_word("hippo", "i"), "h");
    }

    #[test]
    fn make_abba_test() {
        assert_eq!(make_abba("Hi", "Bye"), "HiByeByeHi");
        assert_eq!(make_abba("Yo", "Alice"), "YoAliceAliceYo");
        assert_eq!(make_abba("What", "Up"), "WhatUpUpWhat");
    }

    #[test]
    fn extra_end_test() {
        assert_eq!(extra_end("Hello"), "lololo");
        assert_eq!(extra_end("ab"), "ababab");
        assert_eq!(extra_end("Hi"), "HiHiHi");
    }
}
