// Recursion-1 > factorial
// https://codingbat.com/prob/p154669

// Given n of 1 or more, return the factorial of n, which is n * (n-1) * (n-2) ... 1.
// Compute the result recursively (without loops).

// factorial(1) → 1
// factorial(2) → 2
// factorial(3) → 6

fn factorial(n: u32) -> u32 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Recursion-1 > bunnyEars2
// https://codingbat.com/prob/p107330

// We have bunnies standing in a line, numbered 1, 2, ...
// The odd bunnies (1, 3, ..) have the normal 2 ears.
// The even bunnies (2, 4, ..) we'll say have 3 ears, because they each have a raised foot.
// Recursively return the number of "ears" in the bunny line 1, 2, ... n (without loops or multiplication).

// bunny_ears2(0) → 0
// bunny_ears2(1) → 2
// bunny_ears2(2) → 5

fn bunny_ears2(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        if n % 2 == 1 {
            2 + bunny_ears2(n - 1) // n is odd
        } else {
            3 + bunny_ears2(n - 1) //n is even
        }
    }
}

// Recursion-1 > count7
// https://codingbat.com/prob/p101409

// Given a non-negative int n, return the count of the occurrences of 7 as a digit,
// so for example 717 yields 2. (no loops). Note that mod (%) by 10 yields the rightmost digit (126 % 10 is 6),
// while divide (/) by 10 removes the rightmost digit (126 / 10 is 12).

// count7(717) → 2
// count7(7) → 1
// count7(123) → 0

fn count7(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        if n % 10 == 7 {
            1 + count7(n / 10)
        } else {
            0 + count7(n / 10)
        }
    }
}

// Recursion-1 > countX
// https://codingbat.com/prob/p170371

// Given a string, compute recursively (no loops) the number of lowercase 'x' chars in the string.

// count_x("xxhixx") → 4
// count_x("xhixhix") → 3
// count_x("hi") → 0

fn count_x(s: &str) -> u32 {
    if s.len() == 0 {
        0
    } else {
        if &s[..1] == "x" {
            1 + count_x(&s[1..])
        } else {
            0 + count_x(&s[1..])
        }
    }
}

// Recursion-1 > changePi
// https://codingbat.com/prob/p170924

// Given a string, compute recursively (no loops) a new string where all appearances of "pi" have been replaced by "3.14".

// change_pi("xpix") → "x3.14x"
// change_pi("pipi") → "3.143.14"
// change_pi("pip") → "3.14p"

fn change_pi(s: &str) -> String {
    if s.len() < 2 {
        s.to_string()
    } else {
        if &s[..2] == "pi" {
            format!("{}{}", "3.14", change_pi(&s[2..]))
        } else {
            format!("{}{}", &s[..1], change_pi(&s[1..]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_test() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
    }

    #[test]
    fn bunny_ears2_test() {
        assert_eq!(bunny_ears2(0), 0);
        assert_eq!(bunny_ears2(1), 2);
        assert_eq!(bunny_ears2(2), 5);
    }

    #[test]
    fn count7_test() {
        assert_eq!(count7(717), 2);
        assert_eq!(count7(7), 1);
        assert_eq!(count7(123), 0);
    }

    #[test]
    fn count_x_test() {
        assert_eq!(count_x("xxhixx"), 4);
        assert_eq!(count_x("xhixhix"), 3);
        assert_eq!(count_x("hi"), 0);
    }

    #[test]
    fn change_pi_test() {
        assert_eq!(change_pi("xpix"), "x3.14x");
        assert_eq!(change_pi("pipi"), "3.143.14");
        assert_eq!(change_pi("pip"), "3.14p");
    }
}
