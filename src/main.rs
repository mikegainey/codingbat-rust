#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

// Suppose the string "yak" is unlucky. Given a string, return a version where all the "yak" are removed,
// but the "a" can be any char. The "yak" strings will not overlap.

// string_yak("yakpak") → "pak"
// string_yak("pakyak") → "pak"
// string_yak("yak123ya") → "123ya"

fn main() {
}

fn string_yak(s: &str) -> String {
    s.to_string()
}

fn is_yak(s: &str) -> bool {
    if s.len() < 3 {
        return false
    }
    let mut s_chars = s.chars();
    let y = s_chars.next().unwrap() == 'y';
    s_chars.next();
    let k = s_chars.next().unwrap() == 'k';
    y && k
}
