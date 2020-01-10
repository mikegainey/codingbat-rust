// Warmup-1 > sleepIn
// https://codingbat.com/prob/p187868

// The parameter weekday is true if it is a weekday, and the parameter vacation is true if we are on vacation.
// We sleep in if it is not a weekday or we're on vacation. Return true if we sleep in.

// sleep_in(false, false) == true
// sleep_in(true, false) == false
// sleep_in(false, true) == true

fn sleep_in(weekday: bool, vacation: bool) -> bool {
    vacation || !weekday
}

// Warmup-1 > diff21
// https://codingbat.com/prob/p116624

// Given an int n, return the absolute difference between n and 21,
// except return double the absolute difference if n is over 21.

// diff21(19) → 2
// diff21(10) → 11
// diff21(21) → 0

fn diff21(n: i32) -> i32 {
    let absolute_difference = (n - 21).abs();

    if n <= 21 {
        absolute_difference
    } else {
        absolute_difference * 2
    }
}

// Warmup-1 > nearHundred
// https://codingbat.com/prob/p184004

// Given an int n, return true if it is within 10 of 100 or 200. Note: Math.abs(num) computes the absolute value of a number.

// nearHundred(93) → true
// nearHundred(90) → true
// nearHundred(89) → false

fn near_hundred(n: i32) -> bool {
    // ((n - 100).abs() <= 10) || ((n - 200).abs() <= 10) // this works too
    (n >= 90) & (n <= 110) || (n >= 190) & (n <= 210)
}

// Warmup-1 > missingChar
// https://codingbat.com/prob/p190570

// Given a non-empty string and an int n, return a new string where the char at index n has been removed.
// The value of n will be a valid index of a char in the original string
//     (i.e. n will be in the range 0..str.length()-1 inclusive).

// missing_char("kitten", 1) → "ktten"
// missing_char("kitten", 0) → "itten"
// missing_char("kitten", 4) → "kittn"

fn missing_char(s: &str, n: usize) -> String {
    let take = &s[..n].to_string();
    let drop = &s[n+1..].to_string();
    format!("{}{}", take, drop)
}

// Warmup-1 > backAround
// https://codingbat.com/prob/p161642

// Given a string, take the last char and return a new string with the last char added at the front and back,
// so "cat" yields "tcatt". The original string will be length 1 or more.

// back_around("cat") → "tcatt"
// back_around("Hello") → "oHelloo"
// back_around("a") → "aaa"

fn back_around(s: &str) -> String {
    // let lastx = s.len() - 1;
    // let last_char = &s[lastx..lastx+1];
    let last_char = &s.chars().last().unwrap(); // will panic on an empty string
    format!("{}{}{}", last_char, s, last_char)
}
// Warmup-1 > startHi
// https://codingbat.com/prob/p191022

// Given a string, return true if the string starts with "hi" and false otherwise.

// start_hi("hi there") → true
// start_hi("hi") → true
// start_hi("hello hi") → false

fn start_hi(s: &str) -> bool {
    &s[..2] == "hi"
}

// Warmup-1 > hasTeen
// https://codingbat.com/prob/p178986

// We'll say that a number is "teen" if it is in the range 13..19 inclusive.
// Given 3 int values, return true if 1 or more of them are teen.

// has_teen(13, 20, 10) → true
// has_teen(20, 19, 10) → true
// has_teen(20, 10, 13) → true

fn has_teen(a: i32, b: i32, c: i32) -> bool {

    fn is_teen(x: i32) -> bool {
        (x >= 13) && (x <= 19)
    }

    is_teen(a) || is_teen(b) || is_teen(c)
}

// Warmup-1 > mixStart
// https://codingbat.com/prob/p151713

// Return true if the given string begins with "mix", except the 'm' can be anything, so "pix", "9ix" .. all count.

// mix_start("mix snacks") → true
// mix_start("pix snacks") → true
// mix_start("piz snacks") → false

fn mix_start(s: &str) -> bool {
    &s[1..3] == "ix"
}
// Warmup-1 > close10
// https://codingbat.com/prob/p172021

// Given 2 int values, return whichever value is nearest to the value 10, or return 0 in the event of a tie.
// Note that Math.abs(n) returns the absolute value of a number.

// close10(8, 13) → 8
// close10(13, 8) → 8
// close10(13, 7) → 0

fn close10(a: i32, b: i32) -> i32 {

    fn dist10(x: i32) -> i32 {
        (x - 10).abs()
    }

    let adist = dist10(a);
    let bdist = dist10(b);

    if adist < bdist {
        a
    } else if bdist < adist {
        b
    } else {
        0
    }
}
// Warmup-1 > stringE
// https://codingbat.com/prob/p173784

// Return true if the given string contains between 1 and 3 'e' chars.

// string_e("Hello") → true
// string_e("Heelle") → true
// string_e("Heelele") → false

fn string_e1(s: &str) -> bool {
    let count = s.chars()
        .filter(|&c| c == 'e') // I can put & on the first c, * on the second c, or & on the 'e' and it will work
        .count();              // which is most correct?

    (count >= 1) && (count <= 3)
}

fn string_e2(s: &str) -> bool {
    let count = s.chars()
        .filter(|c| *c == 'e') // I can put & on the first c, * on the second c, or & on the 'e' and it will work
        .count();              // which is most correct?

    (count >= 1) && (count <= 3)
}

fn string_e3(s: &str) -> bool {
    let count = s.chars()
        .filter(|c| c == &'e') // I can put & on the first c, * on the second c, or & on the 'e' and it will work
        .count();              // which is most correct?

    (count >= 1) && (count <= 3)
}

// Warmup-1 > everyNth
// https://codingbat.com/prob/p196441

// Given a non-empty string and an int N,
// return the string made starting with char 0, and then every Nth char of the string.
// So if N is 3, use char 0, 3, 6, ... and so on. N is 1 or more.

// every_nth("Miracle", 2) → "Mrce"
// every_nth("abcdefg", 2) → "aceg"
// every_nth("abcdefg", 3) → "adg"

fn every_nth1(s: &str, n: usize) -> String {
    let mut output = String::new();

    for c in s.chars().step_by(n) {
        output.push(c); // why didn't I have to dereference c?
    }
    output
}

fn every_nth2(s: &str, n: usize) -> String {
    s.chars().step_by(n).collect::<String>()
}

// Warmup-1 > monkeyTrouble
// https://codingbat.com/prob/p181646

// We have two monkeys, a and b, and the parameters aSmile and bSmile indicate if each is smiling.
// We are in trouble if they are both smiling or if neither of them is smiling. Return true if we are in trouble.

// monkey_trouble(true, true) → true
// monkey_trouble(false, false) → true
// monkey_trouble(true, false) → false

fn monkey_trouble(a_smile: bool, b_smile: bool) -> bool {
    a_smile == b_smile
}

// Warmup-1 > parrotTrouble
// https://codingbat.com/prob/p140449

// We have a loud talking parrot. The "hour" parameter is the current hour time in the range 0..23.
// We are in trouble if the parrot is talking and the hour is before 7 or after 20. Return true if we are in trouble.

// parrot_trouble(true, 6) → true
// parrot_trouble(true, 7) → false
// parrot_trouble(false, 6) → false

fn parrot_trouble(talking: bool, hour: u32) -> bool {
    talking && ((hour < 7) || (hour > 20))
}

// Warmup-1 > posNeg
// https://codingbat.com/prob/p159227

// Given 2 int values, return true if one is negative and one is positive.
// Except if the parameter "negative" is true, then return true only if both are negative.

// pos_neg(1, -1, false) → true
// pos_neg(-1, 1, false) → true
// pos_neg(-4, -5, true) → true

fn pos_neg(a: i32, b: i32, negative: bool) -> bool {
    if negative {
        (a < 0) && (b < 0)
    } else {
        (a > 0) != (b > 0)
    }
}

// Warmup-1 > frontBack
// https://codingbat.com/prob/p123384

// Given a string, return a new string where the first and last chars have been exchanged.

// front_back("code") → "eodc"
// front_back("a") → "a"
// front_back("ab") → "ba"

fn front_back1(s: &str) -> String {
    let len = s.len();
    if len == 1 {
        s.to_string()
    } else {
        let front = &s[0..1];       // there has got to be a better way ...
        let back = &s[len-1..len];
        let middle = &s[1..len-1];
        format!("{}{}{}", back, middle, front)
    }
}

fn front_back2(s: &str) -> String {
    let len = s.len();
    if len == 1 {
        s.to_string()
    } else {
        let front = &s.chars().next().unwrap(); // will panic on an empty string slice
        let back = &s.chars().last().unwrap();  // will panic on an empty string slice
        let middle = &s[1..len-1];
        format!("{}{}{}", back, middle, front)
    }
}

// Warmup-1 > or35
// https://codingbat.com/prob/p112564

// Return true if the given non-negative number is a multiple of 3 or a multiple of 5.
// Use the % "mod" operator -- see Introduction to Mod

// or35(3) → true
// or35(10) → true
// or35(8) → false

fn or35(n: i32) -> bool {
    (n % 3 == 0) || (n % 5 == 0)
}

// Warmup-1 > icyHot
// https://codingbat.com/prob/p192082

// Given two temperatures, return true if one is less than 0 and the other is greater than 100.

// icy_hot(120, -1) → true
// icy_hot(-1, 120) → true
// icy_hot(2, 120) → false

fn icy_hot(temp1: i32, temp2: i32) -> bool {
    use std::cmp;

    (cmp::min(temp1, temp2) < 0) && (cmp::max(temp1, temp2) > 100)
}

// Warmup-1 > loneTeen
// https://codingbat.com/prob/p165701

// We'll say that a number is "teen" if it is in the range 13..19 inclusive.
// Given 2 int values, return true if one or the other is teen, but not both.

// lone_teen(13, 99) → true
// lone_teen(21, 19) → true
// lone_teen(13, 13) → false

fn lone_teen(a: i32, b: i32) -> bool {

    fn is_teen(x: i32) -> bool {
        (x >= 13) && (x <= 19)
    }

    is_teen(a) != is_teen(b)
}

// Warmup-1 > startOz
// https://codingbat.com/prob/p199720

// Given a string, return a string made of the first 2 chars (if present),
// however include first char only if it is 'o' and include the second only if it is 'z', so "ozymandias" yields "oz".

// start_oz("ozymandias") → "oz"
// start_oz("bzoo") → "z"
// start_oz("oxx") → "o"

fn start_oz(s: &str) -> String {
    s.chars()
        .zip("oz".chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect::<String>()
}

// Warmup-1 > in3050
// https://codingbat.com/prob/p132134

// Given 2 int values, return true if they are both in the range 30..40 inclusive,
// or they are both in the range 40..50 inclusive.

// in3050(30, 31) → true
// in3050(30, 41) → false
// in3050(40, 50) → true

fn in3050(a: i32, b: i32) -> bool {

    fn in_range(x: i32, lower: i32, upper: i32) -> bool {
        (x >= lower) && (x <= upper)
    }

    (in_range(a, 30, 40) && in_range(b, 30, 40)) || (in_range(a, 40, 50) && in_range(b, 40, 50))
}

// Warmup-1 > lastDigit
// https://codingbat.com/prob/p125339

// Given two non-negative int values, return true if they have the same last digit, such as with 27 and 57.
// Note that the % "mod" operator computes remainders, so 17 % 10 is 7.

// last_digit(7, 17) → true
// last_digit(6, 17) → false
// last_digit(3, 113) → true

fn last_digit(a: i32, b: i32) -> bool {
    (a % 10) == (b % 10)
}

// Warmup-1 > sumDouble
// https://codingbat.com/prob/p154485

// Given two int values, return their sum. Unless the two values are the same, then return double their sum.

// sum_double(1, 2) → 3
// sum_double(3, 2) → 5
// sum_double(2, 2) → 8

fn sum_double(a: i32, b: i32) -> i32 {
    let sum = a + b;

    if a == b {
        sum * 2
    } else {
        sum
    }
}

// Warmup-1 > makes10
// https://codingbat.com/prob/p182873

// Given 2 ints, a and b, return true if one if them is 10 or if their sum is 10.

// makes10(9, 10) → true
// makes10(9, 9) → false
// makes10(1, 9) → true

fn makes10(a: i32, b: i32) -> bool {
    (a == 10) || (b == 10) || (a + b == 10)
}

// Warmup-1 > notString
// https://codingbat.com/prob/p191914

// Given a string, return a new string where "not " has been added to the front.
// However, if the string already begins with "not", return the string unchanged. Note: use .equals() to compare 2 strings.

// not_string("candy") → "not candy"
// not_string("x") → "not x"
// not_string("not bad") → "not bad"

fn not_string(s: &str) -> String {
    if (s.len() < 3) || (&s[..3] != "not") {
        format!("not {}", s)
    } else {
        s.to_string()
    }
}

// Warmup-1 > front3
// https://codingbat.com/prob/p136351

// Given a string, we'll say that the front is the first 3 chars of the string.
// If the string length is less than 3, the front is whatever is there.
// Return a new string which is 3 copies of the front.

// front3("Java") → "JavJavJav"
// front3("Chocolate") → "ChoChoCho"
// front3("abc") → "abcabcabc"

fn front3(s: &str) -> String {
    use std::cmp;

    let len = s.len();
    let lastx = cmp::min(3, len);
    let front = &s[..lastx];

    format!("{0}{0}{0}", front)
}

// Warmup-1 > front22
// https://codingbat.com/prob/p183592

// Given a string, take the first 2 chars and return the string with the 2 chars added at both the front and back,
// so "kitten" yields"kikittenki". If the string length is less than 2, use whatever chars are there.

// front22("kitten") → "kikittenki"
// front22("Ha") → "HaHaHa"
// front22("abc") → "ababcab"

fn front22(s: &str) -> String {
    use std::cmp;

    let len = s.len();
    let lastx = cmp::min(2, len);
    let front = &s[..lastx];

    format!("{0}{1}{0}", front, s)
}

// Warmup-1 > in1020
// https://codingbat.com/prob/p144535

// Given 2 int values, return true if either of them is in the range 10..20 inclusive.

// in1020(12, 99) → true
// in1020(21, 12) → true
// in1020(8, 99) → false

fn in1020(a: i32, b: i32) -> bool {

    fn in_range(x: i32, lower: i32, upper: i32) -> bool {
        (x >= lower) && (x <= upper)
    }

    in_range(a, 10, 20) || in_range(b, 10, 20)
}

// Warmup-1 > delDel
// https://codingbat.com/prob/p100905

// Given a string, if the string "del" appears starting at index 1, return a string where that "del" has been deleted.
// Otherwise, return the string unchanged.

// del_del("adelbc") → "abc"
// del_del("adelHello") → "aHello"
// del_del("adedbc") → "adedbc"

fn del_del(s: &str) -> String {
    if (s.len() > 4) && (&s[1..4] == "del") {
        let front = &s.chars().next().unwrap(); // will panic on an empty string slice
        let rest = &s[4..];
        format!("{}{}", front, rest)
    } else {
        s.to_string()
    }
}

// Warmup-1 > intMax
// https://codingbat.com/prob/p101887

// Given three int values, a b c, return the largest.

// int_max(1, 2, 3) → 3
// int_max(1, 3, 2) → 3
// int_max(3, 2, 1) → 3

fn int_max(a: i32, b: i32, c: i32) -> i32 {
    use std::cmp;

    // cmp::max(cmp::max(a, b), c) // this works, but it's hard to read

    let list = [a, b, c];

    list.iter().fold(a, |acc, &x| cmp::max(acc, x)) // this is cool functional programming!
}

// Warmup-1 > max1020
// https://codingbat.com/prob/p177372

// Given 2 positive int values, return the larger value that is in the range 10..20 inclusive,
// or return 0 if neither is in that range.

// max1020(11, 19) → 19
// max1020(19, 11) → 19
// max1020(11, 9) → 11

fn max1020(a: i32, b: i32) -> i32 {

    fn in_range(x: i32, lower: i32, upper: i32) -> bool {
        (x >= lower) && (x <= upper)
    }

    if (a >= b) && (in_range(a, 10, 20)) {
        a
    } else if in_range(b, 10, 20) {
        b
    } else {
        0
    }
}

// Warmup-1 > endUp
// https://codingbat.com/prob/p125268

// Given a string, return a new string where the last 3 chars are now in upper case.
// If the string has less than 3 chars, uppercase whatever is there.
// Note that str.toUpperCase() returns the uppercase version of a string.

// end_up("Hello") → "HeLLO"
// end_up("hi there") → "hi thERE"
// end_up("hi") → "HI"

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aleep_in_test() {
        assert!(sleep_in(false, false) == true);
        assert!(sleep_in(true, false) == false);
        assert!(sleep_in(false, true) == true);
        assert!(sleep_in(true, true) == true);
    }

    #[test]
    fn diff21_test() {
        assert_eq!(diff21(10), 11);
        assert_eq!(diff21(19), 2);
        assert_eq!(diff21(20), 1);
        assert_eq!(diff21(21), 0);
        assert_eq!(diff21(22), 2);
        assert_eq!(diff21(23), 4);
    }

    #[test]
    fn near_hundred_test() {
        assert_eq!(near_hundred(89), false);
        assert_eq!(near_hundred(90), true);
        assert_eq!(near_hundred(91), true);
        assert_eq!(near_hundred(109), true);
        assert_eq!(near_hundred(110), true);
        assert_eq!(near_hundred(111), false);
        assert_eq!(near_hundred(189), false);
        assert_eq!(near_hundred(190), true);
        assert_eq!(near_hundred(191), true);
        assert_eq!(near_hundred(209), true);
        assert_eq!(near_hundred(210), true);
        assert_eq!(near_hundred(211), false);
    }

    #[test]
    fn missing_char_test() {
        assert_eq!(missing_char("kitten", 0), "itten");
        assert_eq!(missing_char("kitten", 1), "ktten");
        assert_eq!(missing_char("kitten", 4), "kittn");
        assert_eq!(missing_char("kitten", 5), "kitte");
    }

    #[test]
    fn back_around_test() {
        assert_eq!(back_around("cat"), "tcatt");
        assert_eq!(back_around("Hello"), "oHelloo");
        assert_eq!(back_around("a"), "aaa");
    }

    #[test]
    fn start_hi_test() {
        assert_eq!(start_hi("hi there"), true);
        assert_eq!(start_hi("hi"), true);
        assert_eq!(start_hi("hello hi"), false);
    }

    #[test]
    fn has_teen_test() {
        assert_eq!(has_teen(13, 20, 10), true);
        assert_eq!(has_teen(20, 19, 10), true);
        assert_eq!(has_teen(20, 10, 13), true);
    }

    #[test]
    fn mix_start_test() {
        assert_eq!(mix_start("mix snacks"), true);
        assert_eq!(mix_start("pix snacks"), true);
        assert_eq!(mix_start("piz snacks"), false);
    }

    #[test]
    fn close10_test() {
        assert_eq!(close10(8, 13), 8);
        assert_eq!(close10(13, 8), 8);
        assert_eq!(close10(13, 7), 0);
    }

    #[test]
    fn string_e_test() {
        assert_eq!(string_e1("Hallo"), false);
        assert_eq!(string_e1("Hello"), true);
        assert_eq!(string_e1("elephant"), true);
        assert_eq!(string_e1("Heelle"), true);
        assert_eq!(string_e1("Heelele"), false);
        assert_eq!(string_e2("Hallo"), false);
        assert_eq!(string_e2("Hello"), true);
        assert_eq!(string_e2("elephant"), true);
        assert_eq!(string_e2("Heelle"), true);
        assert_eq!(string_e2("Heelele"), false);
        assert_eq!(string_e3("Hallo"), false);
        assert_eq!(string_e3("Hello"), true);
        assert_eq!(string_e3("elephant"), true);
        assert_eq!(string_e3("Heelle"), true);
        assert_eq!(string_e3("Heelele"), false);
    }

    #[test]
    fn every_nth_test() {
        assert_eq!(every_nth1("Miracle", 2), "Mrce");
        assert_eq!(every_nth1("abcdefg", 2), "aceg");
        assert_eq!(every_nth1("abcdefg", 3), "adg");
        assert_eq!(every_nth2("Miracle", 2), "Mrce");
        assert_eq!(every_nth2("abcdefg", 2), "aceg");
        assert_eq!(every_nth2("abcdefg", 3), "adg");
    }

    #[test]
    fn monkey_trouble_test() {
        assert_eq!(monkey_trouble(true, true), true);
        assert_eq!(monkey_trouble(false, false), true);
        assert_eq!(monkey_trouble(true, false), false);
    }

    #[test]
    fn parrot_trouble_test() {
        assert_eq!(parrot_trouble(false, 6), false);
        assert_eq!(parrot_trouble(true, 6), true);
        assert_eq!(parrot_trouble(true, 7), false);
        assert_eq!(parrot_trouble(false, 7), false);
        assert_eq!(parrot_trouble(false, 20), false);
        assert_eq!(parrot_trouble(true, 20), false);
        assert_eq!(parrot_trouble(true, 21), true);
        assert_eq!(parrot_trouble(false, 21), false);
    }

    #[test]
    fn pos_neg_test() {
        assert_eq!(pos_neg(1, -1, false), true);
        assert_eq!(pos_neg(-1, 1, false), true);
        assert_eq!(pos_neg(-1, -1, false), false);
        assert_eq!(pos_neg(1, 1, false), false);
        assert_eq!(pos_neg(-4, -5, true), true);
        assert_eq!(pos_neg(-4, 5, true), false);
        assert_eq!(pos_neg(4, -5, true), false);
        assert_eq!(pos_neg(4, 5, true), false);
    }

    #[test]
    fn front_back_test() {
        assert_eq!(front_back1("code"), "eodc");
        assert_eq!(front_back1("a"), "a");
        assert_eq!(front_back1("ab"), "ba");
        assert_eq!(front_back2("code"), "eodc");
        assert_eq!(front_back2("a"), "a");
        assert_eq!(front_back2("ab"), "ba");
    }

    #[test]
    fn or35_test() {
        assert_eq!(or35(3), true);
        assert_eq!(or35(10), true);
        assert_eq!(or35(8), false);
    }

    #[test]
    fn icy_hot_test() {
        assert_eq!(icy_hot(120, -1), true);
        assert_eq!(icy_hot(-1, 120), true);
        assert_eq!(icy_hot(2, 120), false);
    }

    #[test]
    fn lone_teen_test() {
        assert_eq!(lone_teen(13, 99), true);
        assert_eq!(lone_teen(21, 19), true);
        assert_eq!(lone_teen(13, 13), false);
        assert_eq!(lone_teen(20, 12), false);
    }

    #[test]
    fn start_oz_test() {
        assert_eq!(start_oz("ozymandias"), "oz");
        assert_eq!(start_oz("bzoo"), "z");
        assert_eq!(start_oz("oxx"), "o");
    }

    #[test]
    fn in3050_test() {
        assert_eq!(in3050(30, 31), true);
        assert_eq!(in3050(30, 41), false);
        assert_eq!(in3050(40, 50), true);
    }

    #[test]
    fn last_digit_test() {
        assert_eq!(last_digit(7, 17), true);
        assert_eq!(last_digit(6, 17), false);
        assert_eq!(last_digit(3, 113), true);
    }

    #[test]
    fn sum_double_test() {
        assert_eq!(sum_double(1, 2), 3);
        assert_eq!(sum_double(3, 2), 5);
        assert_eq!(sum_double(2, 2), 8);
    }

    #[test]
    fn makes10_test() {
        assert_eq!(makes10(9, 10), true);
        assert_eq!(makes10(9, 9), false);
        assert_eq!(makes10(1, 9), true);
    }

    #[test]
    fn not_string_test() {
        assert_eq!(not_string("candy"), "not candy");
        assert_eq!(not_string("x"), "not x");
        assert_eq!(not_string("not bad"), "not bad");
    }

    #[test]
    fn front3_test() {
        assert_eq!(front3("Java"), "JavJavJav");
        assert_eq!(front3("Chocolate"), "ChoChoCho");
        assert_eq!(front3("abc"), "abcabcabc");
    }

    #[test]
    fn front22_test() {
        assert_eq!(front22("kitten"), "kikittenki");
        assert_eq!(front22("Ha"), "HaHaHa");
        assert_eq!(front22("abc"), "ababcab");
    }

    #[test]
    fn in1020_test() {
        assert_eq!(in1020(12, 99), true);
        assert_eq!(in1020(21, 12), true);
        assert_eq!(in1020(8, 99), false);
    }

    #[test]
    fn del_del_test() {
        assert_eq!(del_del("adelbc"), "abc");
        assert_eq!(del_del("adelHello"), "aHello");
        assert_eq!(del_del("adedbc"), "adedbc");
        assert_eq!(del_del("a"), "a");
    }

    #[test]
    fn int_max_test() {
        assert_eq!(int_max(1, 2, 3), 3);
        assert_eq!(int_max(1, 3, 2), 3);
        assert_eq!(int_max(3, 2, 1), 3);
    }

    #[test]
    fn max1020_test() {
        assert_eq!(max1020(11, 19), 19);
        assert_eq!(max1020(19, 11), 19);
        assert_eq!(max1020(11, 9), 11);
        assert_eq!(max1020(9, 11), 11);
        assert_eq!(max1020(21, 9), 0);
    }

    #[test]
    fn end_up_test() {
        assert_eq!(end_up("Hello"), "HeLLO");
        assert_eq!(end_up("hi there"), "hi thERE");
        assert_eq!(end_up("hi"), "HI");
    }
}
