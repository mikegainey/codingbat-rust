// Logic-1 > cigarParty
// https://codingbat.com/prob/p159531

// When squirrels get together for a party, they like to have cigars.
// A squirrel party is successful when the number of cigars is between 40 and 60, inclusive.
// Unless it is the weekend, in which case there is no upper bound on the number of cigars.
// Return true if the party with the given values is successful, or false otherwise.

// cigar_party(30, false) → false
// cigar_party(50, false) → true
// cigar_party(70, true) → true

fn cigar_party(cigars: u32, is_weekend: bool) -> bool {
    if is_weekend {
        cigars >= 40
    } else {
        cigars >= 40 && cigars <= 60
    }
}

// Logic-1 > caughtSpeeding
// https://codingbat.com/prob/p157733

// You are driving a little too fast, and a police officer stops you.
// Write code to compute the result, encoded as an int value: 0=no ticket, 1=small ticket, 2=big ticket.
// If speed is 60 or less, the result is 0.
// If speed is between 61 and 80 inclusive, the result is 1.
// If speed is 81 or more, the result is 2.
// Unless it is your birthday -- on that day, your speed can be 5 higher in all cases.

// caught_speeding(60, false) → 0
// caught_speeding(65, false) → 1
// caught_speeding(65, true) → 0

fn caught_speeding(speed: i32, is_birthday: bool) -> u8 {
    let birthday_allowance = if is_birthday {
        -5
    } else {
        0
    };
    if (speed + birthday_allowance) <= 60 {
        0
    } else if (speed + birthday_allowance) <= 80 {
        1
    } else {
        2
    }
}

// Logic-1 > love6
// https://codingbat.com/prob/p137742

// The number 6 is a truly great number. Given two int values, a and b, return true if either one is 6.
// Or if their sum or difference is 6. Note: the function Math.abs(num) computes the absolute value of a number.

// love6(6, 4) → true
// love6(4, 5) → false
// love6(1, 5) → true

fn love6(a: i32, b: i32) -> bool {
    let sum = a + b;
    let abs_diff = (a + b).abs();
    a == 6 || b == 6 || sum == 6 || abs_diff == 6
}

// Logic-1 > more20
// https://codingbat.com/prob/p118290

// Return true if the given non-negative number is 1 or 2 more than a multiple of 20.

// more20(20) → false
// more20(21) → true
// more20(22) → true

fn more20(a: u32) -> bool {
    let a_mod20 = a % 20;
    a_mod20 == 1 || a_mod20 == 2
}

// Logic-1 > nearTen
// https://codingbat.com/prob/p193613

// Given a non-negative number "num", return true if num is within 2 of a multiple of 10.
// Note: (a % b) is the remainder of dividing a by b, so (7 % 5) is 2. See also: Introduction to Mod

// near_ten(12) → true
// near_ten(17) → false
// near_ten(19) → true

fn near_ten(a: u32) -> bool {
    for remainder in [0,1,2,8,9].iter() {
        if a % 10 == *remainder {
            return true;
        }
    }
    false
}

// Logic-1 > teaParty
// https://codingbat.com/prob/p177181

// We are having a party with amounts of tea and candy.
// Return the int outcome of the party encoded as 0=bad, 1=good, or 2=great.
// A party is good (1) if both tea and candy are at least 5.
// However, if either tea or candy is at least double the amount of the other one, the party is great (2).
// However, in all cases, if either tea or candy is less than 5, the party is always bad (0).

// tea_party(6, 8) → 1
// tea_party(3, 8) → 0
// tea_party(20, 6) → 2

fn tea_party(tea: u32, candy: u32) -> u32 {
    use std::cmp;

    if tea < 5 || candy < 5 {
        0 // bad
    } else if cmp::max(tea, candy) >= 2 * cmp::min(tea, candy) {
        2 // great
    } else {
        1 // good
    }
}

// Logic-1 > twoAsOne
// https://codingbat.com/prob/p113261

// Given three ints, a b c, return true if it is possible to add two of the ints to get the third.

// two_as_one(1, 2, 3) → true
// two_as_one(3, 1, 2) → true
// two_as_one(3, 2, 2) → false

fn two_as_one(a: i32, b: i32, c: i32) -> bool {
    a + b == c || a + c == b || b + c == a
}

// Logic-1 > lastDigit
// https://codingbat.com/prob/p169213

// Given three ints, a b c, return true if two or more of them have the same rightmost digit.
// The ints are non-negative. Note: the % "mod" operator computes the remainder, e.g. 17 % 10 is 7.

// last_digit(23, 19, 13) → true
// last_digit(23, 19, 12) → false
// last_digit(23, 19, 3) → true

fn last_digit(a: u32, b: u32, c: u32) -> bool {
    let a_ones = a % 10;
    let b_ones = b % 10;
    let c_ones = c % 10;
    a_ones == b_ones || a_ones == c_ones || b_ones == c_ones
}

// Logic-1 > maxMod5
// https://codingbat.com/prob/p115384

// Given two int values, return whichever value is larger.
// However if the two values have the same remainder when divided by 5, then the return the smaller value.
// However, in all cases, if the two values are the same, return 0.
// Note: the % "mod" operator computes the remainder, e.g. 7 % 5 is 2.

// max_mod5(2, 3) → 3
// max_mod5(6, 2) → 6
// max_mod5(3, 2) → 3

fn max_mod5(a: i32, b: i32) -> i32 {
    use std::cmp;

    if a == b {
        0
    } else if a % 5 == b % 5 {
        cmp::min(a, b)
    } else {
        cmp::max(a, b)
    }
}

// Logic-1 > blueTicket
// https://codingbat.com/prob/p192267

// You have a blue lottery ticket, with ints a, b, and c on it. This makes three pairs, which we'll call ab, bc, and ac.
// Consider the sum of the numbers in each pair. If any pair sums to exactly 10, the result is 10.
// Otherwise if the ab sum is exactly 10 more than either bc or ac sums, the result is 5.
// Otherwise the result is 0.

// blue_ticket(9, 1, 0) → 10
// blue_ticket(9, 2, 0) → 0
// blue_ticket(6, 1, 4) → 10

fn blue_ticket(a: i32, b: i32, c: i32) -> i32 {
    let ab = a + b;
    let bc = b + c;
    let ac = a + c;
    if ab == 10 || bc == 10 || ac == 10 {
        10
    } else if ab == bc + 10 || ab == ac + 10 {
        5
    } else {
        0
    }
}

// Logic-1 > dateFashion
// https://codingbat.com/prob/p103360

// You and your date are trying to get a table at a restaurant.
// The parameter "you" is the stylishness of your clothes, in the range 0..10,
// and "date" is the stylishness of your date's clothes.
// The result getting the table is encoded as an int value with 0=no, 1=maybe, 2=yes.
// If either of you is very stylish, 8 or more, then the result is 2 (yes).
// With the exception that if either of you has style of 2 or less, then the result is 0 (no).
// Otherwise the result is 1 (maybe).

// date_fashion(5, 10) → 2
// date_fashion(5, 2) → 0
// date_fashion(5, 5) → 1

fn date_fashion(you: u8, date: u8) -> u8 {
    if you <= 2 || date <= 2 {
        0
    } else if you >= 8 || date >= 8 {
        2
    } else {
        1
    }
}

// Logic-1 > sortaSum
// https://codingbat.com/prob/p183071

// Given 2 ints, a and b, return their sum.
// However, sums in the range 10..19 inclusive, are forbidden, so in that case just return 20.

// sorta_sum(3, 4) → 7
// sorta_sum(9, 4) → 20
// sorta_sum(10, 11) → 21

fn sorta_sum(a: i32, b: i32) -> i32 {
    let sum = a + b;
    if sum >= 10 && sum <= 19 {
        20
    } else {
        sum
    }
}

// Logic-1 > in1To10
// https://codingbat.com/prob/p137365

// Given a number n, return true if n is in the range 1..10, inclusive.
// Unless outsideMode is true, in which case return true if the number is less or equal to 1, or greater or equal to 10.

// in_1to10(5, false) → true
// in_1to10(11, false) → false
// in_1to10(11, true) → true

fn in_1to10(n: i32, outside_mode: bool) -> bool {
    if outside_mode {
        n <= 1 || n >= 10
    } else {
        n >= 1 && n <= 10
    }
}

// Logic-1 > old35
// https://codingbat.com/prob/p159612

// Return true if the given non-negative number is a multiple of 3 or 5, but not both.
// Use the % "mod" operator -- see Introduction to Mod

// old35(3) → true
// old35(10) → true
// old35(15) → false

fn old35(n: u32) -> bool {
    (n % 3 == 0) != (n % 5 == 0)
}

// Logic-1 > teenSum
// https://codingbat.com/prob/p178728

// Given 2 ints, a and b, return their sum. However, "teen" values in the range 13..19 inclusive, are extra lucky.
// So if either value is a teen, just return 19.

// teen_sum(3, 4) → 7
// teen_sum(10, 13) → 19
// teen_sum(13, 2) → 19

fn teen_sum(a: i32, b: i32) -> i32 {

    fn is_teen(a: i32) -> bool {
        a >= 13 && a <= 19
    }

    if is_teen(a) || is_teen(b) {
        19
    } else {
        a + b
    }
}

// Logic-1 > fizzString
// https://codingbat.com/prob/p137136

// Given a string str, if the string starts with "f" return "Fizz".
// If the string ends with "b" return "Buzz".
// If both the "f" and "b" conditions are true, return "FizzBuzz".
// In all other cases, return the string unchanged. (See also: FizzBuzz Code)

// fizz_string("fig") → "Fizz"
// fizz_string("dib") → "Buzz"
// fizz_string("fib") → "FizzBuzz"

fn fizz_string(s: &str) -> String {
    let bs = s.as_bytes(); // this is nice: turn the string into bytes, then no more pain
    let end = bs.len() - 1;
    if bs[0] == b'f' && bs[end] == b'b' {
        "FizzBuzz".to_string()
    } else if bs[0] == b'f' {
        "Fizz".to_string()
    } else if bs[end] == b'b' {
        "Buzz".to_string()
    } else {
        s.to_string()
    }
}

// Logic-1 > inOrder
// https://codingbat.com/prob/p154188

// Given three ints, a b c, return true if b is greater than a, and c is greater than b.
// However, with the exception that if "bOk" is true, b does not need to be greater than a.

// in_order(1, 2, 4, false) → true
// in_order(1, 2, 1, false) → false
// in_order(1, 1, 2, true) → true

fn in_order(a: i32, b: i32, c: i32, b_ok: bool) -> bool {
    if !(c > b) {
        return false;
    }
    if b_ok {
        return true;
    } else {
        b > a
    }
}

// Logic-1 > lessBy10
// https://codingbat.com/prob/p179196

// Given three ints, a b c, return true if one of them is 10 or more less than one of the others.

// less_by_10(1, 7, 11) → true
// less_by_10(1, 7, 10) → false
// less_by_10(11, 1, 7) → true

fn less_by_10(a: i32, b: i32, c: i32) -> bool {
    use std::cmp;

    let min = cmp::min(cmp::min(a, b), c);
    let max = cmp::max(cmp::max(a, b), c);

    max - min >= 10
}

// Logic-1 > redTicket
// https://codingbat.com/prob/p170833

// You have a red lottery ticket showing ints a, b, and c, each of which is 0, 1, or 2.
// If they are all the value 2, the result is 10.
// Otherwise if they are all the same, the result is 5.
// Otherwise so long as both b and c are different from a, the result is 1.
// Otherwise the result is 0.

// red_ticket(2, 2, 2) → 10
// red_ticket(2, 2, 1) → 0
// red_ticket(0, 0, 0) → 5

fn red_ticket(a: i32, b: i32, c: i32) -> i32 {
    if a == 2 && b == 2 && c == 2 {
        10
    } else if a == b && b == c {
        5
    } else if b != a && c != a {
        1
    } else {
        0
    }
}

// Logic-1 > shareDigit
// https://codingbat.com/prob/p153748

// Given two ints, each in the range 10..99,
// return true if there is a digit that appears in both numbers, such as the 2 in 12 and 23.
// (Note: division, e.g. n/10, gives the left digit while the % "mod" n%10 gives the right digit.)

// share_digit(12, 23) → true
// share_digit(12, 43) → false
// share_digit(12, 44) → false

fn share_digit(a: i32, b: i32) -> bool {
    if a % 10 == b % 10 {
        true
    } else if a % 10 == b / 10 {
        true
    } else if a / 10 == b % 10 {
        true
    } else if a / 10 == b / 10 {
        true
    } else {
        false
    }
}

fn share_digit2(a: i32, b: i32) -> bool {
    let mut va = Vec::new();
    let mut vb = Vec::new();
    va.push(a / 10);
    va.push(a % 10);
    vb.push(b / 10);
    vb.push(b % 10);
    let mut match_found = false;
    for x in &va {
        for y in &vb {
            if x == y {
                match_found = true;
            }
        }
    }
    match_found
}

// Logic-1 > squirrelPlay
// https://codingbat.com/prob/p141061

// The squirrels in Palo Alto spend most of the day playing.
// In particular, they play if the temperature is between 60 and 90 (inclusive).
// Unless it is summer, then the upper limit is 100 instead of 90.
// Given an int temperature and a boolean isSummer, return true if the squirrels play and false otherwise.

// squirrel_play(70, false) → true
// squirrel_play(95, false) → false
// squirrel_play(95, true) → true

fn squirrel_play(temp: i32, is_summer: bool) -> bool {
    if is_summer {
        temp >= 60 && temp <= 100
    } else {
        temp >= 60 && temp <= 90 // not summer
    }
}

// Logic-1 > alarmClock
// https://codingbat.com/prob/p160543

// Given a day of the week encoded as 0=Sun, 1=Mon, 2=Tue, ...6=Sat,
// and a boolean indicating if we are on vacation,
// return a string of the form "7:00" indicating when the alarm clock should ring.
// Weekdays, the alarm should be "7:00" and on the weekend it should be "10:00".
// Unless we are on vacation -- then on weekdays it should be "10:00" and weekends it should be "off".

// alarm_clock(1, false) → "7:00"
// alarm_clock(5, false) → "7:00"
// alarm_clock(0, false) → "10:00"

fn alarm_clock(day: i32, vacation: bool) -> String {
    if vacation {
        if day >= 1 && day <= 5 {
            "10:00".to_string()
        } else { // weekend
            "off".to_string()
        }
    } else { // not on vacation
        if day >= 1 && day <= 5 {
            "7:00".to_string()
        } else { // weekend
            "10:00".to_string()
        }
    }
}

// Logic-1 > specialEleven
// https://codingbat.com/prob/p100962

// We'll say a number is special if it is a multiple of 11 or if it is one more than a multiple of 11.
// Return true if the given non-negative number is special. Use the % "mod" operator -- see Introduction to Mod

// special_eleven(22) → true
// special_eleven(23) → true
// special_eleven(24) → false

fn special_eleven(n: u32) -> bool {
    n % 11 == 0 || n % 11 == 1
}

// Logic-1 > less20
// https://codingbat.com/prob/p133158

// Return true if the given non-negative number is 1 or 2 less than a multiple of 20.
// So for example 38 and 39 return true, but 40 returns false. See also: Introduction to Mod

// less20(18) → true
// less20(19) → true
// less20(20) → false

fn less20(n: u32) -> bool {
    n % 20 == 19 || n % 20 == 18
}

// Logic-1 > answerCell
// https://codingbat.com/prob/p110973

// Your cell phone rings. Return true if you should answer it.
// Normally you answer, except in the morning you only answer if it is your mom calling.
// In all cases, if you are asleep, you do not answer.

// answer_cell(false, false, false) → true
// answer_cell(false, false, true) → false
// answer_cell(true, false, false) → false

fn answer_cell(is_morning: bool, is_mom: bool, is_asleep: bool) -> bool {
    if is_asleep {
        false
    } else if is_morning && !is_mom {
        false
    } else {
        true
    }
}

// Logic-1 > fizzString2
// https://codingbat.com/prob/p115243

// Given an int n, return the string form of the number followed by "!". So the int 6 yields "6!".
// Except if the number is divisible by 3 use "Fizz" instead of the number,
// and if the number is divisible by 5 use "Buzz",
// and if divisible by both 3 and 5, use "FizzBuzz".

// fizz_string2(1) → "1!"
// fizz_string2(2) → "2!"
// fizz_string2(3) → "Fizz"

fn fizz_string2(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        format!("{}!", n)
    }
}

// Logic-1 > inOrderEqual
// https://codingbat.com/prob/p140272

// Given three ints, a b c, return true if they are in strict increasing order,
// such as 2 5 11, or 5 6 7, but not 6 5 7 or 5 5 7.
// However, with the exception that if "equalOk" is true, equality is allowed, such as 5 5 7 or 5 5 5.

// in_order_equal(2, 5, 11, false) → true
// in_order_equal(5, 7, 6, false) → false
// in_order_equal(5, 5, 7, true) → true

fn in_order_equal(a: i32, b: i32, c: i32, equal_ok: bool) -> bool {
    if equal_ok {
        a <= b && b <= c
    } else {
        a < b && b < c
    }
}

// Logic-1 > withoutDoubles
// https://codingbat.com/prob/p115233

// Return the sum of two 6-sided dice rolls, each in the range 1..6.
// However, if noDoubles is true, if the two dice show the same value, increment one die to the next value,
// wrapping around to 1 if its value was 6.

// without_doubles(2, 3, true) → 5
// without_doubles(3, 3, true) → 7
// without_doubles(3, 3, false) → 6

fn without_doubles(die1: u8, mut die2: u8, no_doubles: bool) -> u8 {
    if no_doubles && die1 == die2 {
        die2 += 1;
        if die2 == 7 {
            die2 = 1
        }
        return die1 + die2;
    } else {
        return die1 + die2;
    }
}

// Logic-1 > greenTicket
// https://codingbat.com/prob/p120633

// You have a green lottery ticket, with ints a, b, and c on it.
// If the numbers are all different from each other, the result is 0.
// If all of the numbers are the same, the result is 20.
// If two of the numbers are the same, the result is 10.

// green_ticket(1, 2, 3) → 0
// green_ticket(2, 2, 2) → 20
// green_ticket(1, 1, 2) → 10

fn green_ticket(a: i32, b: i32, c: i32) -> i32 {
    if a == b && b == c {
        20
    } else if a == b || b == c || a == c {
        10
    } else {
        0
    }
}

// Logic-1 > sumLimit
// https://codingbat.com/prob/p118077

// Given 2 non-negative ints, a and b, return their sum, so long as the sum has the same number of digits as a.
// If the sum has more digits than a, just return a without b.

// sum_limit(2, 3) → 5
// sum_limit(8, 3) → 8
// sum_limit(8, 1) → 9

fn sum_limit(a: u32, b: u32) -> u32 {
    let len_a = a.to_string().len();
    let sum = a + b;
    let len_sum = sum.to_string().len();
    if len_sum == len_a {
        sum
    } else {
        a
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cigar_party_test() {
    assert_eq!(cigar_party(30, false), false);
    assert_eq!(cigar_party(50, false), true);
    assert_eq!(cigar_party(70, true), true);
    }

    #[test]
    fn caught_speeding_test() {
        assert_eq!(caught_speeding(60, false), 0);
        assert_eq!(caught_speeding(65, false), 1);
        assert_eq!(caught_speeding(65, true), 0);
    }

    #[test]
    fn love6_test() {
        assert_eq!(love6(6, 4), true);
        assert_eq!(love6(4, 5), false);
        assert_eq!(love6(1, 5), true);
    }

    #[test]
    fn more20_test() {
        assert_eq!(more20(20), false);
        assert_eq!(more20(21), true);
        assert_eq!(more20(22), true);
        assert_eq!(more20(40), false);
        assert_eq!(more20(41), true);
        assert_eq!(more20(42), true);
    }

    #[test]
    fn near_ten_test() {
        assert_eq!(near_ten(12), true);
        assert_eq!(near_ten(17), false);
        assert_eq!(near_ten(19), true);
    }

    #[test]
    fn tea_party_test() {
        assert_eq!(tea_party(6, 8), 1);
        assert_eq!(tea_party(3, 8), 0);
        assert_eq!(tea_party(20, 6), 2);
    }

    #[test]
    fn two_as_one_test() {
        assert_eq!(two_as_one(1, 2, 3), true);
        assert_eq!(two_as_one(3, 1, 2), true);
        assert_eq!(two_as_one(3, 2, 2), false);
    }

    #[test]
    fn last_digit_test() {
        assert_eq!(last_digit(23, 19, 13), true);
        assert_eq!(last_digit(23, 19, 12), false);
        assert_eq!(last_digit(23, 19, 3), true);
    }

    #[test]
    fn max_mod5_test() {
        assert_eq!(max_mod5(2, 3), 3);
        assert_eq!(max_mod5(6, 2), 6);
        assert_eq!(max_mod5(3, 2), 3);
    }

    #[test]
    fn blue_ticket_test() {
        assert_eq!(blue_ticket(9, 1, 0), 10);
        assert_eq!(blue_ticket(9, 2, 0), 0);
        assert_eq!(blue_ticket(6, 1, 4), 10);
    }

    #[test]
    fn date_fashion_test() {
        assert_eq!(date_fashion(5, 10), 2);
        assert_eq!(date_fashion(5, 2), 0);
        assert_eq!(date_fashion(5, 5), 1);
    }

    #[test]
    fn sorta_sum_test() {
        assert_eq!(sorta_sum(3, 4), 7);
        assert_eq!(sorta_sum(9, 4), 20);
        assert_eq!(sorta_sum(10, 11), 21);
    }
    #[test]
    fn in_1to10_test() {
        assert_eq!(in_1to10(5, false), true);
        assert_eq!(in_1to10(11, false), false);
        assert_eq!(in_1to10(11, true), true);
    }

    #[test]
    fn old35_test() {
        assert_eq!(old35(3), true);
        assert_eq!(old35(10), true);
        assert_eq!(old35(15), false);
    }

    #[test]
    fn teen_sum_test() {
        assert_eq!(teen_sum(3, 4), 7);
        assert_eq!(teen_sum(10, 13), 19);
        assert_eq!(teen_sum(13, 2), 19);
    }

    #[test]
    fn fizz_string_test() {
        assert_eq!(fizz_string("fig"), "Fizz");
        assert_eq!(fizz_string("dib"), "Buzz");
        assert_eq!(fizz_string("fib"), "FizzBuzz");
    }

    #[test]
    fn in_order_test() {
        assert_eq!(in_order(1, 2, 4, false), true);
        assert_eq!(in_order(1, 2, 1, false), false);
        assert_eq!(in_order(1, 1, 2, true), true);
        assert_eq!(in_order(1, 1, 2, false), false);
    }

    #[test]
    fn less_by_10_test() {
        assert_eq!(less_by_10(1, 7, 11), true);
        assert_eq!(less_by_10(1, 7, 10), false);
        assert_eq!(less_by_10(11, 1, 7), true);
    }

    #[test]
    fn red_ticket_test() {
        assert_eq!(red_ticket(2, 2, 2), 10);
        assert_eq!(red_ticket(2, 2, 1), 0);
        assert_eq!(red_ticket(0, 0, 0), 5);
        assert_eq!(red_ticket(2, 3, 1), 1);
    }

    #[test]
    fn share_digit_test() {
        assert_eq!(share_digit(12, 23), true);
        assert_eq!(share_digit(12, 43), false);
        assert_eq!(share_digit(12, 44), false);
    }

    #[test]
    fn share_digit2_test() {
        assert_eq!(share_digit2(12, 23), true);
        assert_eq!(share_digit2(12, 43), false);
        assert_eq!(share_digit2(12, 44), false);
    }

    #[test]
    fn squirrel_play_test() {
        assert_eq!(squirrel_play(70, false), true);
        assert_eq!(squirrel_play(95, false), false);
        assert_eq!(squirrel_play(95, true), true);
    }

    #[test]
    fn alarm_clock_test() {
        assert_eq!(alarm_clock(1, false), "7:00");
        assert_eq!(alarm_clock(5, false), "7:00");
        assert_eq!(alarm_clock(0, false), "10:00");
    }

    #[test]
    fn special_eleven_test() {
        assert_eq!(special_eleven(22), true);
        assert_eq!(special_eleven(23), true);
        assert_eq!(special_eleven(24), false);
    }

    #[test]
    fn less20_test() {
        assert_eq!(less20(18), true);
        assert_eq!(less20(19), true);
        assert_eq!(less20(20), false);
    }

    #[test]
    fn answer_cell_test() {
        assert_eq!(answer_cell(false, false, false), true);
        assert_eq!(answer_cell(false, false, true), false);
        assert_eq!(answer_cell(true, false, false), false);
    }

    #[test]
    fn fizz_string2_test() {
        assert_eq!(fizz_string2(1), "1!");
        assert_eq!(fizz_string2(2), "2!");
        assert_eq!(fizz_string2(3), "Fizz");
    }

    #[test]
    fn in_order_equal_test() {
        assert_eq!(in_order_equal(2, 5, 11, false), true);
        assert_eq!(in_order_equal(5, 7, 6, false), false);
        assert_eq!(in_order_equal(5, 5, 7, true), true);
    }

    #[test]
    fn without_doubles_test() {
        assert_eq!(without_doubles(2, 3, true), 5);
        assert_eq!(without_doubles(3, 3, true), 7);
        assert_eq!(without_doubles(3, 3, false), 6);
        assert_eq!(without_doubles(6, 6, false), 12);
        assert_eq!(without_doubles(6, 6, true), 7);
    }

    #[test]
    fn green_ticket_test() {
        assert_eq!(green_ticket(1, 2, 3), 0);
        assert_eq!(green_ticket(2, 2, 2), 20);
        assert_eq!(green_ticket(1, 1, 2), 10);
    }

    #[test]
    fn sum_limit_test() {
        assert_eq!(sum_limit(2, 3), 5);
        assert_eq!(sum_limit(8, 3), 8);
        assert_eq!(sum_limit(8, 1), 9);
        assert_eq!(sum_limit(80, 19), 99);
        assert_eq!(sum_limit(80, 20), 80);
    }
}
