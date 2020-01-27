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
}
