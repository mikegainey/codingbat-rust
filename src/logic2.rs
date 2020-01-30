// Logic-2 > makeBricks
// https://codingbat.com/prob/p183562

// We want to make a row of bricks that is goal inches long.
// We have a number of small bricks (1 inch each) and big bricks (5 inches each).
// Return true if it is possible to make the goal by choosing from the given bricks.
// This is a little harder than it looks and can be done without any loops. See also: Introduction to MakeBricks

// make_bricks(3, 1, 8) → true
// make_bricks(3, 1, 9) → false
// make_bricks(3, 2, 10) → true

fn make_bricks(small: u32, big: u32, goal: u32) -> bool {
    let big_needed = goal / 5;
    let small_needed = goal % 5;
    big >= big_needed && small >= small_needed
}

// Logic-2 > noTeenSum
// https://codingbat.com/prob/p182879

// Given 3 int values, a b c, return their sum.
// However, if any of the values is a teen -- in the range 13..19 inclusive -- then that value counts as 0,
// except 15 and 16 do not count as a teens.
// Write a separate helper "public int fixTeen(int n) {"
// that takes in an int value and returns that value fixed for the teen rule.
// In this way, you avoid repeating the teen code 3 times (i.e. "decomposition").

// no_teen_sum(1, 2, 3) → 6
// no_teen_sum(2, 13, 1) → 3
// no_teen_sum(2, 1, 14) → 3

fn no_teen_sum(a: i32, b: i32, c: i32) -> i32 {

    fn fix_teen(n: i32) -> i32 {
        match n {
            13 => 0,
            14 => 0,
            17 => 0,
            18 => 0,
            19 => 0,
            _ => n
        }
    }

    fix_teen(a) + fix_teen(b) + fix_teen(c)
}

// Logic-2 > blackjack
// https://codingbat.com/prob/p117019

// Given 2 int values greater than 0, return whichever value is nearest to 21 without going over.
// Return 0 if they both go over.

// blackjack(19, 21) → 21
// blackjack(21, 19) → 21
// blackjack(19, 22) → 19

fn blackjack(a: u32, b: u32) -> u32 {
    use std::cmp;

    if a > 21 && b > 21 {
        0
    } else if a > 21 {
        b
    } else if b > 21 {
        a
    } else {
        cmp::max(a, b)
    }
}

// Logic-2 > loneSum
// https://codingbat.com/prob/p148972

// Given 3 int values, a b c, return their sum.
// However, if one of the values is the same as another of the values, it does not count towards the sum.

// lone_sum(1, 2, 3) → 6
// lone_sum(3, 2, 3) → 2
// lone_sum(3, 3, 3) → 0

fn lone_sum(a: i32, b: i32, c: i32) -> i32 {
    if a == b && b == c {
        0
    } else if a == b {
        c
    } else if b == c {
        a
    } else if a == c {
        b
    } else {
        a + b + c
    }
}

// Logic-2 > roundSum
// https://codingbat.com/prob/p186753

// For this problem, we'll round an int value up to the next multiple of 10 if its rightmost digit is 5 or more,
// so 15 rounds up to 20. Alternately, round down to the previous multiple of 10 if its rightmost digit is less than 5,
// so 12 rounds down to 10.
// Given 3 ints, a b c, return the sum of their rounded values.
// To avoid code repetition, write a separate helper "public int round10(int num) {" and call it 3 times.

// round_sum(16, 17, 18) → 60
// round_sum(12, 13, 14) → 30
// round_sum(6, 4, 4) → 10

fn round_sum(a: i32, b: i32, c: i32) -> i32 {

    fn round10(n: i32) -> i32 {
        let n_mod10 = n % 10;
        if n_mod10 >= 5 {
            n + (10 - n_mod10)
        } else {
            n - n_mod10
        }
    }

    round10(a) + round10(b) + round10(c)
}

// Logic-2 > evenlySpaced
// https://codingbat.com/prob/p198700

// Given three ints, a b c, one of them is small, one is medium and one is large.
// Return true if the three values are evenly spaced,
// so the difference between small and medium is the same as the difference between medium and large.

// evenly_spaced(2, 4, 6) → true
// evenly_spaced(4, 6, 2) → true
// evenly_spaced(4, 6, 3) → false

// find the min and max values; if the difference is odd, there is no middle
// compute what the middle should be (max - min) / 2
// check each value to see if it's the middle

fn evenly_spaced(a: i32, b: i32, c: i32) -> bool {
    use std::cmp;

    let min = cmp::min(cmp::min(a, b), c);
    let max = cmp::max(cmp::max(a, b), c);
    let span = max - min;

    if span % 2 == 1 {
        false // if span is odd, there is no middle
    } else {
        let middle = min + (span / 2);            // compute what the middle should be
        a == middle || b == middle || c == middle // check to see if it's there
    }
}

// Logic-2 > luckySum
// https://codingbat.com/prob/p130788

// Given 3 int values, a b c, return their sum.
// However, if one of the values is 13 then it does not count towards the sum and values to its right do not count.
// So for example, if b is 13, then both b and c do not count.

// lucky_sum(1, 2, 3) → 6
// lucky_sum(1, 2, 13) → 3
// lucky_sum(1, 13, 3) → 1

fn lucky_sum(a: i32, b: i32, c: i32) -> i32 {
    if a == 13 {
        0
    } else if b == 13 {
        a
    } else if c == 13 {
        a + b
    } else {
        a + b + c
    }
}

// Logic-2 > closeFar
// https://codingbat.com/prob/p138990

// Given three ints, a b c, return true if one of b or c is "close" (differing from a by at most 1),
// while the other is "far", differing from both other values by 2 or more.

// close_far(1, 2, 10) → true
// close_far(1, 2, 3) → false
// close_far(4, 1, 3) → true

fn close_far(a: i32, b: i32, c: i32) -> bool {
    let b_close = (a - b).abs() <= 1;
    let c_close = (a - c).abs() <= 1;
    let b_far = (b - a).abs() >= 2 && (b - c).abs() >= 2;
    let c_far = (c - a).abs() >= 2 && (c - b).abs() >= 2;

    b_close && c_far || c_close && b_far
}

// Logic-2 > makeChocolate
// https://codingbat.com/prob/p191363

// We want make a package of goal kilos of chocolate. We have small bars (1 kilo each) and big bars (5 kilos each).
// Return the number of small bars to use, assuming we always use big bars before small bars.
// Return -1 if it can't be done.

// make_chocolate(4, 1, 9) → 4
// make_chocolate(4, 1, 10) → -1
// make_chocolate(4, 1, 7) → 2

fn make_chocolate(small: u32, big: u32, goal: u32) -> i32 {
    let big_needed = goal / 5;
    if big < big_needed {
        return -1;
    }
    let small_needed = goal % 5;
    if small < small_needed {
        return -1;
    }
    small_needed as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_bricks_test() {
        assert_eq!(make_bricks(3, 1, 8), true);
        assert_eq!(make_bricks(3, 1, 9), false);
        assert_eq!(make_bricks(3, 2, 10), true);
    }

    #[test]
    fn no_teen_sum_test() {
        assert_eq!(no_teen_sum(1, 2, 3), 6);
        assert_eq!(no_teen_sum(2, 13, 1), 3);
        assert_eq!(no_teen_sum(2, 1, 14), 3);
    }

    #[test]
    fn blackjack_test() {
        assert_eq!(blackjack(19, 21), 21);
        assert_eq!(blackjack(21, 19), 21);
        assert_eq!(blackjack(9, 22), 9);
        assert_eq!(blackjack(22, 1), 1);
        assert_eq!(blackjack(23, 24), 0);
    }

    #[test]
    fn lone_sum_test() {
        assert_eq!(lone_sum(1, 2, 3), 6);
        assert_eq!(lone_sum(3, 2, 3), 2);
        assert_eq!(lone_sum(3, 3, 3), 0);
        assert_eq!(lone_sum(3, 3, 4), 4);
        assert_eq!(lone_sum(3, 4, 4), 3);
    }

    #[test]
    fn round_sum_test() {
        assert_eq!(round_sum(16, 17, 18), 60);
        assert_eq!(round_sum(12, 13, 14), 30);
        assert_eq!(round_sum(6, 4, 4), 10);
    }

    #[test]
    fn evenly_spaced_test() {
        assert_eq!(evenly_spaced(2, 4, 6), true);
        assert_eq!(evenly_spaced(4, 6, 2), true);
        assert_eq!(evenly_spaced(4, 6, 3), false);
        assert_eq!(evenly_spaced(4, 0, 8), true);
        assert_eq!(evenly_spaced(50, 0, 24), false);
        assert_eq!(evenly_spaced(50, 0, 25), true);
    }

    #[test]
    fn lucky_sum_test() {
        assert_eq!(lucky_sum(1, 2, 3), 6);
        assert_eq!(lucky_sum(1, 2, 13), 3);
        assert_eq!(lucky_sum(1, 13, 3), 1);
    }

    #[test]
    fn close_far_test() {
        assert_eq!(close_far(1, 2, 10), true);
        assert_eq!(close_far(1, 2, 3), false);
        assert_eq!(close_far(4, 1, 3), true);
    }

    #[test]
    fn make_chocolate_test() {
        assert_eq!(make_chocolate(4, 1, 9), 4);
        assert_eq!(make_chocolate(4, 1, 10), -1);
        assert_eq!(make_chocolate(4, 1, 7), 2);
        assert_eq!(make_chocolate(4, 4, 17), 2);
        assert_eq!(make_chocolate(1, 4, 17), -1);
    }
}
