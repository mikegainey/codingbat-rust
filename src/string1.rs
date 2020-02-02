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

fn first_half(s: &str) -> String {
    let len = s.len();
    s[..len/2].to_string()
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

fn the_end(s: &str, front: bool) -> String {
    if front {
        s[..1].to_string()
    } else {
        let end = s.len()-1;
        s[end..].to_string()
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
}
