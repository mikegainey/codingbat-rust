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
    if s.starts_with("red") {
        &s[..3]
    } else if s.starts_with("blue") {
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

// String-1 > withoutEnd
// https://codingbat.com/prob/p130896

// Given a string, return a version without the first and last char, so "Hello" yields "ell".
// The string length will be at least 2.

// without_end("Hello") → "ell"
// without_end("java") → "av"
// without_end("coding") → "odin"

fn without_end(s: &str) -> &str {
    let len = s.len();
    &s[1..len-1]
}

// String-1 > left2
// https://codingbat.com/prob/p197720

// Given a string, return a "rotated left 2" version where the first 2 chars are moved to the end.
// The string length will be at least 2.

// left2("Hello") → "lloHe"
// left2("java") → "vaja"
// left2("Hi") → "Hi"

fn left2(s: &str) -> String {
    format!("{}{}", &s[2..], &s[..2])
}

// String-1 > withouEnd2
// https://codingbat.com/prob/p174254

// Given a string, return a version without both the first and last char of the string.
// The string may be any length, including 0.

// withou_end2("Hello") → "ell"
// withou_end2("abc") → "b"
// withou_end2("ab") → ""

fn withou_end2(s: &str) -> &str {
    let len = s.len();
    if len < 2 {
        ""
    } else {
        &s[1..len-1]
    }
}

// String-1 > nTwice
// https://codingbat.com/prob/p174148

// Given a string and an int n, return a string made of the first and last n chars from the string.
// The string length will be at least n.

// n_twice("Hello", 2) → "Helo"
// n_twice("Chocolate", 3) → "Choate"
// n_twice("Chocolate", 1) → "Ce"

fn n_twice(s: &str, n: usize) -> String {
    let len = s.len();
    let front = &s[..n];
    let back = &s[len-n..];
    format!("{}{}", front, back)
}

// String-1 > hasBad
// https://codingbat.com/prob/p139075

// Given a string, return true if "bad" appears starting at index 0 or 1 in the string,
// such as with "badxxx" or "xbadxx" but not "xxbadxx".
// The string may be any length, including 0.

// has_bad("badxx") → true
// has_bad("xbadxx") → true
// has_bad("xxbadxx") → false

fn has_bad(s: &str) -> bool {
    &s[..3] == "bad" || &s[1..4] == "bad"
}

// String-1 > conCat
// https://codingbat.com/prob/p132118

// Given two strings, append them together (known as "concatenation") and return the result.
// However, if the concatenation creates a double-char, then omit one of the chars, so "abc" and "cat" yields "abcat".

// con_cat("abc", "cat") → "abcat"
// con_cat("dog", "cat") → "dogcat"
// con_cat("abc", "") → "abc"

fn con_cat(a: &str, b: &str) -> String {
    let (alen, blen) = (a.len(), b.len());
    if alen == 0 {
        b.to_string()
    } else if blen == 0 {
        a.to_string()
      // compare the last character of a with the first character of b
    } else if a.chars().last().unwrap() == b.chars().next().unwrap() {
        format!("{}{}", &a, &b[1..])
    } else {
        format!("{}{}", &a, &b)
    }
}

// String-1 > frontAgain
// https://codingbat.com/prob/p196652

// Given a string, return true if the first 2 chars in the string also appear at the end of the string,
// such as with "edited".

// front_again("edited") → true
// front_again("edit") → false
// front_again("ed") → true

fn front_again(s: &str) -> bool {
    let len = s.len();
    s[..2] == s[len-2..]
}

// String-1 > without2
// https://codingbat.com/prob/p142247

// Given a string, if a length 2 substring appears at both its beginning and end,
// return a string without the substring at the beginning, so "HelloHe" yields "lloHe".
// The substring may overlap with itself, so "Hi" yields "". Otherwise, return the original string unchanged.

// without2("HelloHe") → "lloHe"
// without2("HelloHi") → "HelloHi"
// without2("Hi") → ""

fn without2(s: &str) -> &str {
    let len = s.len();
    if s[..2] == s[len-2..] {
        &s[2..]
    } else {
        s
    }
}

// String-1 > withoutX
// https://codingbat.com/prob/p151940

// Given a string, if the first or last chars are 'x', return the string without those 'x' chars,
// and otherwise return the string unchanged.

// without_x("xHix") → "Hi"
// without_x("xHi") → "Hi"
// without_x("Hxix") → "Hxi"

fn without_x(s: &str) -> &str {
    if !s.is_ascii() {
        panic!("non-ascii input");
    }
    let len = s.len();
    let front_x = s.starts_with('x');
    let back_x = s.ends_with('x');
    if front_x && back_x {
        &s[1..len-1]
    } else if front_x {
        &s[1..]
    } else if back_x {
        &s[..len-1]
    } else {
        s
    }
}

// String-1 > makeTags
// https://codingbat.com/prob/p147483

// The web is built with HTML strings like "<i>Yay</i>" which draws Yay as italic text.
// In this example, the "i" tag makes <i> and </i> which surround the word "Yay".
// Given tag and word strings, create the HTML string with tags around the word, e.g. "<i>Yay</i>".

// make_tags("i", "Yay") → "<i>Yay</i>"
// make_tags("i", "Hello") → "<i>Hello</i>"
// make_tags("cite", "Yay") → "<cite>Yay</cite>"

fn make_tags(tag: &str, word: &str) -> String {
    format!("<{0}>{1}</{0}>", tag, word)
}

// String-1 > firstTwo
// https://codingbat.com/prob/p163411

// Given a string, return the string made of its first two chars, so the String "Hello" yields "He".
// If the string is shorter than length 2, return whatever there is,
// so "X" yields "X", and the empty string "" yields the empty string "".

// first_two("Hello") → "He"
// first_two("abcdefg") → "ab"
// first_two("ab") → "ab"

fn first_two(s: &str) -> &str {
    use std::cmp;

    let len = cmp::min(2, s.len());
    &s[..len]
}

// String-1 > comboString
// https://codingbat.com/prob/p168564

// Given 2 strings, a and b, return a string of the form short+long+short,
// with the shorter string on the outside and the longer string on the inside.
// The strings will not be the same length, but they may be empty (length 0).

// combo_string("Hello", "hi") → "hiHellohi"
// combo_string("hi", "Hello") → "hiHellohi"
// combo_string("aaa", "b") → "baaab"

fn combo_string(a: &str, b: &str) -> String {
    if a.len() < b.len() {
        format!("{0}{1}{0}", a, b)
    } else {
        format!("{0}{1}{0}", b, a)
    }
}

// String-1 > right2
// https://codingbat.com/prob/p130781

// Given a string, return a "rotated right 2" version where the last 2 chars are moved to the start.
// The string length will be at least 2.

// right2("Hello") → "loHel"
// right2("java") → "vaja"
// right2("Hi") → "Hi"

fn right2(s: &str) -> String {
    let len = s.len();
    format!("{}{}", &s[len-2..], &s[..len-2])
}

// String-1 > middleTwo
// https://codingbat.com/prob/p137729

// Given a string of even length, return a string made of the middle two chars,
// so the string "string" yields "ri". The string length will be at least 2.

// middle_two("string") → "ri"
// middle_two("code") → "od"
// middle_two("Practice") → "ct"

fn middle_two(s: &str) -> &str {
    let midx = s.len() / 2;
    &s[midx-1..midx+1]
}

// String-1 > twoChar
// https://codingbat.com/prob/p144623

// Given a string and an index, return a string length 2 starting at the given index.
// If the index is too big or too small to define a string length 2, use the first 2 chars.
// The string length will be at least 2.

// two_char("java", 0) → "ja"
// two_char("java", 2) → "va"
// two_char("java", 3) → "ja"

fn two_char(s: &str, index: usize) -> &str {
    let len = s.len();
    if index > len - 2 {
        &s[..2]
    } else {
        &s[index..index+2]
    }
}

// String-1 > atFirst
// https://codingbat.com/prob/p139076

// Given a string, return a string length 2 made of its first 2 chars.
// If the string length is less than 2, use '@' for the missing chars.

// at_first("hello") → "he"
// at_first("hi") → "hi"
// at_first("h") → "h@"

fn at_first(s: &str) -> String {
    let s_len = std::cmp::min(2, s.len());
    let at_len = 2 - s_len;
    let ats = "@".repeat(at_len);
    format!("{}{}", &s[..s_len], ats)
}

// String-1 > lastTwo
// https://codingbat.com/prob/p194786

// Given a string of any length, return a new string where the last 2 chars, if present, are swapped,
// so "coding" yields "codign".

// last_two("coding") → "codign"
// last_two("cat") → "cta"
// last_two("ab") → "ba"

fn last_two(s: &str) -> String {
    let len = s.len();
    let s_bytes = s.as_bytes();
    let end1 = s_bytes[len-1] as char;
    let end2 = s_bytes[len-2] as char;
    format!("{}{}{}", &s[..len-2], end1, end2)
}

// String-1 > minCat
// https://codingbat.com/prob/p105745

// Given two strings, append them together (known as "concatenation") and return the result.
// However, if the strings are different lengths,
// omit chars from the longer string so it is the same length as the shorter string.
// So "Hello" and "Hi" yield "loHi". The strings may be any length.

// min_cat("Hello", "Hi") → "loHi"
// min_cat("Hello", "java") → "ellojava"
// min_cat("java", "Hello") → "javaello"

fn min_cat(a: &str, b: &str) -> String {
    let (lena, lenb) = (a.len(), b.len());
    if lena < lenb {
        format!("{}{}", a, &b[lenb-lena..])
    } else {
        format!("{}{}", &a[lena-lenb..], b)
    }
}

// String-1 > deFront
// https://codingbat.com/prob/p110141

// Given a string, return a version without the first 2 chars.
// Except: keep the first char if it is 'a'
// and   : keep the second char if it is 'b'.
// The string may be any length. Harder than it looks.

// de_front("Hello") → "llo"
// de_front("java") → "va"
// de_front("away") → "aay"

fn de_front(s: &str) -> String {
    let s_bytes = s.as_bytes();
    let first = match s_bytes[0] {
        b'a' => "a",
        _ => ""
    };
    let second = match s_bytes[1] {
        b'b' => "b",
        _ => ""
    };
    format!("{}{}{}", first, second, &s[2..])
}

// String-1 > withoutX2
// https://codingbat.com/prob/p151359

// Given a string, if one or both of the first 2 chars is 'x',
// return the string without those 'x' chars, and
// otherwise return the string unchanged.
// This is a little harder than it looks.

// without_x2("xHi") → "Hi"
// without_x2("Hxi") → "Hi"
// without_x2("Hi") → "Hi"

fn without_x2(s: &str) -> String {
    let first2: String = s[..2].chars()
        .filter(|c| c != &'x')
        .collect();
    let rest = &s[2..];
    format!("{}{}", first2, rest)
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

    #[test]
    fn without_end_test() {
        assert_eq!(without_end("Hello"), "ell");
        assert_eq!(without_end("java"), "av");
        assert_eq!(without_end("coding"), "odin");
    }

    #[test]
    fn left2_test() {
        assert_eq!(left2("Hello"), "lloHe");
        assert_eq!(left2("java"), "vaja");
        assert_eq!(left2("Hi"), "Hi");
    }

    #[test]
    fn withou_end2_test() {
        assert_eq!(withou_end2("Hello"), "ell");
        assert_eq!(withou_end2("abc"), "b");
        assert_eq!(withou_end2("ab"), "");
    }

    #[test]
    fn n_twice_test() {
        assert_eq!(n_twice("Hello", 2), "Helo");
        assert_eq!(n_twice("Chocolate", 3), "Choate");
        assert_eq!(n_twice("Chocolate", 1), "Ce");
    }

    #[test]
    fn has_bad_test() {
        assert_eq!(has_bad("badxx"), true);
        assert_eq!(has_bad("xbadxx"), true);
        assert_eq!(has_bad("xxbadxx"), false);
    }

    #[test]
    fn con_cat_test() {
        assert_eq!(con_cat("abc", "cat"), "abcat");
        assert_eq!(con_cat("dog", "cat"), "dogcat");
        assert_eq!(con_cat("abc", ""), "abc");
    }

    #[test]
    fn front_again_test() {
        assert_eq!(front_again("edited"), true);
        assert_eq!(front_again("edit"), false);
        assert_eq!(front_again("ed"), true);
    }

    #[test]
    fn without2_test() {
        assert_eq!(without2("HelloHe"), "lloHe");
        assert_eq!(without2("HelloHi"), "HelloHi");
        assert_eq!(without2("Hi"), "");
    }

    #[test]
    fn without_x_test() {
        assert_eq!(without_x("xHix"), "Hi");
        assert_eq!(without_x("xHi"), "Hi");
        assert_eq!(without_x("Hxix"), "Hxi");
    }

    #[test]
    fn make_tags_test() {
        assert_eq!(make_tags("i", "Yay"), "<i>Yay</i>");
        assert_eq!(make_tags("i", "Hello"), "<i>Hello</i>");
        assert_eq!(make_tags("cite", "Yay"), "<cite>Yay</cite>");
    }

    #[test]
    fn first_two_test() {
        assert_eq!(first_two("Hello"), "He");
        assert_eq!(first_two("abcdefg"), "ab");
        assert_eq!(first_two("ab"), "ab");
    }

    #[test]
    fn combo_string_test() {
        assert_eq!(combo_string("Hello", "hi"), "hiHellohi");
        assert_eq!(combo_string("hi", "Hello"), "hiHellohi");
        assert_eq!(combo_string("aaa", "b"), "baaab");
    }

    #[test]
    fn right2_test() {
        assert_eq!(right2("Hello"), "loHel");
        assert_eq!(right2("java"), "vaja");
        assert_eq!(right2("Hi"), "Hi");
    }

    #[test]
    fn middle_two_test() {
        assert_eq!(middle_two("string"), "ri");
        assert_eq!(middle_two("code"), "od");
        assert_eq!(middle_two("Practice"), "ct");
    }

    #[test]
    fn two_char_test() {
        assert_eq!(two_char("java", 0), "ja");
        assert_eq!(two_char("java", 2), "va");
        assert_eq!(two_char("java", 3), "ja");
    }

    #[test]
    fn at_first_test() {
        assert_eq!(at_first("hello"), "he");
        assert_eq!(at_first("hi"), "hi");
        assert_eq!(at_first("h"), "h@");
    }

    #[test]
    fn last_two_test() {
        assert_eq!(last_two("coding"), "codign");
        assert_eq!(last_two("cat"), "cta");
        assert_eq!(last_two("ab"), "ba");
    }

    #[test]
    fn min_cat_test() {
        assert_eq!(min_cat("Hello", "Hi"), "loHi");
        assert_eq!(min_cat("Hello", "java"), "ellojava");
        assert_eq!(min_cat("java", "Hello"), "javaello");
    }

    #[test]
    fn de_front_test() {
        assert_eq!(de_front("Hello"), "llo");
        assert_eq!(de_front("java"), "va");
        assert_eq!(de_front("away"), "aay");
    }

    #[test]
    fn without_x2_test() {
        assert_eq!(without_x2("xHi"), "Hi");
        assert_eq!(without_x2("Hxi"), "Hi");
        assert_eq!(without_x2("Hi"), "Hi");
    }
}
