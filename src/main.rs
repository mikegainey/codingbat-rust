#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

fn main() {
    println!("{}", test("michael", ""));
}


fn test<'a>(string: &'a str, word: &'a str) -> &'a str {
    for (x, (s, w)) in string.chars().zip(word.chars()).enumerate() {
        println!("{} {} {}", x, s, w);
        if s != w && x > 0 {
            return &string[..x];
        }
    }
    &string[..word.len()]
}

