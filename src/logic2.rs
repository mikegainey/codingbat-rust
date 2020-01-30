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
}
