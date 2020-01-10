#![allow(dead_code)]

fn main() {
    println!("{}", end_up("michael"));
}

fn end_up(s: &str) -> String {
    let len = s.len();
    let firstx = if len < 3 {
        0
    } else {
        len - 3
    };

    let front = s[..firstx].to_string();
    let back = s[firstx..].to_uppercase().to_string();

    format!("{}{}", front, back)
}

// Warmup-1 > endUp
// https://codingbat.com/prob/p125268

// Given a string, return a new string where the last 3 chars are now in upper case.
// If the string has less than 3 chars, uppercase whatever is there.
// Note that str.toUpperCase() returns the uppercase version of a string.

// end_up("Hello") → "HeLLO"
// end_up("hi there") → "hi thERE"
// end_up("hi") → "HI"
