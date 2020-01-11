#![allow(dead_code)]

fn main() {
    println!("{}", end_up("michael"));
}

fn missing_char(s: &str, n: usize) -> String {
    let take = &s[..n].to_string();
    let drop = &s[n+1..].to_string();
    format!("{}{}", take, drop)
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
