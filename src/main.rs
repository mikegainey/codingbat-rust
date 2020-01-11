#![allow(dead_code)]

fn main() {
    println!("{}", last2("mimimiaemimi"));
}

fn last2(s: &str) -> u32 {
    let len = s.len();
    let substring = &s[len-2..];
    let search_string = &s[..len-2];

    fn count2(search_string: &str, substring: &str) -> u32 {
        if search_string.len() < 2 {
            0
        } else {
            if &search_string[0..2] == substring {
                1 + count2(&search_string[1..], &substring)
            } else {
                0 + count2(&search_string[1..], &substring)
            }
        }
    }

    count2(&search_string, &substring)
}
