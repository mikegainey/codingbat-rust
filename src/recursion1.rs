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

// Recursion-1 > array11
// https://codingbat.com/prob/p135988

// Given an array of ints, compute recursively the number of times that the value 11 appears in the array.
// We'll use the convention of considering only the part of the array that begins at the given index.
// In this way, a recursive call can pass index+1 to move down the array. The initial call will pass in index as 0.

// array11([1, 2, 11], 0) → 1
// array11([11, 11], 0) → 2
// array11([1, 2, 3, 4], 0) → 0

fn array11(a: &[i32], index: usize) -> u32 {
    if index >= a.len() {
        0
    } else {
        if a[index] == 11 {
            1 + array11(&a, index+1)
        } else {
            0 + array11(&a, index+1)
        }
    }
}

// Recursion-1 > pairStar
// https://codingbat.com/prob/p158175

// Given a string, compute recursively a new string
// where identical chars that are adjacent in the original string are separated from each other by a "*".

// pair_star("hello") → "hel*lo"
// pair_star("xxyy") → "x*xy*y"
// pair_star("aaaa") → "a*a*a*a"

fn pair_star(s: &str) -> String {
    if s.len() < 2 {
        s.to_string()
    } else {
        let head = &s[..2].as_bytes();
        if head[0] == head[1] {
            // two identical chars
            format!("{}*{}", head[0] as char, pair_star(&s[1..]))
        } else {
            // two different chars
            format!("{}{}", head[0] as char, pair_star(&s[1..]))
        }
    }
}

// Recursion-1 > countAbc
// https://codingbat.com/prob/p161124

// Count recursively the total number of "abc" and "aba" substrings that appear in the given string.

// count_abc("abc") → 1
// count_abc("abcxxabc") → 2
// count_abc("abaxxaba") → 2

fn count_abc(s: &str) -> u32 {
    if s.len() < 3 {
        0
    } else {
        let head = &s[..3].as_bytes();
        if head[0] == b'a' && (head[2] == b'a' || head[2] == b'c') {
            1 + count_abc(&s[3..])
        } else {
            0 + count_abc(&s[1..])
        }
    }
}

// Recursion-1 > countHi2
// https://codingbat.com/prob/p143900

// Given a string, compute recursively the number of times lowercase "hi" appears in the string,
// however do not count "hi" that have an 'x' immedately before them.

// count_hi2("ahixhi") → 1
// count_hi2("ahibhi") → 2
// count_hi2("xhixhi") → 0

fn count_hi2(s: &str) -> u32 {

    fn helper(s: &str, previous: Option<char>) -> u32 {
        if s.len() < 2 {
            0
        } else {
            if &s[..2] == "hi" && previous != Some('x') {
                1 + helper(&s[1..], Some('h'))
            } else {
                0 + helper(&s[1..], Some(s.chars().next().unwrap()))
            }
        }
    }
    helper(&s, None) // previous starts out as None
}

// Recursion-1 > strCount
// https://codingbat.com/prob/p186177

// Given a string and a non-empty substring sub,
// compute recursively the number of times that sub appears in the string, without the sub strings overlapping.

// str_count("catcowcat", "cat") → 2
// str_count("catcowcat", "cow") → 1
// str_count("catcowcat", "dog") → 0

fn str_count(s: &str, sub: &str) -> u32 {
    let sublen = sub.len();
    if s.len() < sublen {
        0
    } else {
        if &s[..sublen] == sub {
            1 + str_count(&s[sublen..], &sub)
        } else {
            0 + str_count(&s[1..], &sub)
        }
    }
}

// Recursion-1 > bunnyEars
// https://codingbat.com/prob/p183649

// We have a number of bunnies and each bunny has two big floppy ears.
// We want to compute the total number of ears across all the bunnies recursively (without loops or multiplication).

// bunny_ears(0) → 0
// bunny_ears(1) → 2
// bunny_ears(2) → 4

fn bunny_ears(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        2 + bunny_ears(n-1)
    }
}

// Recursion-1 > triangle
// https://codingbat.com/prob/p194781

// We have triangle made of blocks.
// The topmost row has 1 block, the next row down has 2 blocks, the next row has 3 blocks, and so on.
// Compute recursively the total number of blocks in such a triangle with the given number of rows.

// triangle(0) → 0
// triangle(1) → 1
// triangle(2) → 3

fn triangle(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n + triangle(n-1)
    }
}

// Recursion-1 > count8
// https://codingbat.com/prob/p192383

// Given a non-negative int n, compute recursively (no loops) the count of the occurrences of 8 as a digit,
// except that an 8 with another 8 immediately to its left counts double, so 8818 yields 4.
// Note that mod (%) by 10 yields the rightmost digit (126 % 10 is 6),
// while divide (/) by 10 removes the rightmost digit (126 / 10 is 12).

// count8(8) → 1
// count8(818) → 2
// count8(8818) → 4

// this function gives the wrong answer with 888 (3 or more 8's)
fn count8(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        if n >= 10 && n % 100 == 88 {
            3 + count8(n / 100)
        } else if n % 10 == 8 {
            1 + count8(n / 10)
        } else {
            0 + count8(n / 10)
        }
    }
}

// Recursion-1 > countHi
// https://codingbat.com/prob/p184029

// Given a string, compute recursively (no loops) the number of times lowercase "hi" appears in the string.

// count_hi("xxhixx") → 1
// count_hi("xhixhix") → 2
// count_hi("hi") → 1

fn count_hi(s: &str) -> u32 {
    if s.len() < 2 {
        0
    } else {
        if &s[..2] == "hi" {
            1 + count_hi(&s[2..])
        } else {
            0 + count_hi(&s[1..])
        }
    }
}

// Recursion-1 > noX
// https://codingbat.com/prob/p118230

// Given a string, compute recursively a new string where all the 'x' chars have been removed.

// no_x("xaxb") → "ab"
// no_x("abc") → "abc"
// no_x("xx") → ""

fn no_x(s: &str) -> String {
    if s.len() == 0 {
        "".to_string()
    } else {
        if s.chars().next().unwrap() == 'x' {
            no_x(&s[1..])
        } else {
            format!("{}{}", &s.chars().next().unwrap(), no_x(&s[1..]))
        }
    }
}

// Recursion-1 > array220
// https://codingbat.com/prob/p173469

// Given an array of ints,
// compute recursively if the array contains somewhere a value followed in the array by that value times 10.
// We'll use the convention of considering only the part of the array that begins at the given index.
// In this way, a recursive call can pass index+1 to move down the array. The initial call will pass in index as 0.

// array220([1, 2, 20], 0) → true
// array220([3, 30], 0) → true
// array220([3], 0) → false

fn array220(a: &[i32], index: usize) -> bool {
    if a.len() < 2 {
        false
    } else {
        if a[index+1] == a[index] * 10 {
            true
        } else {
            array220(&a, index+1)
        }
    }
}

// Recursion-1 > endX
// https://codingbat.com/prob/p105722

// Given a string,
// compute recursively a new string where all the lowercase 'x' chars have been moved to the end of the string.

// end_x("xxre") → "rexx"
// end_x("xxhixx") → "hixxxx"
// end_x("xhixhix") → "hihixxx"

fn end_x(s: &str) -> String {
    if s.len() == 0 {
        "".to_string()
    } else {
        if s.chars().next().unwrap() == 'x' {
            format!("{}x", end_x(&s[1..]))
        } else {
            format!("{}{}", s.chars().next().unwrap(), end_x(&s[1..]))
        }
    }
}

// Recursion-1 > count11
// https://codingbat.com/prob/p167015

// Given a string, compute recursively (no loops) the number of "11" substrings in the string.
// The "11" substrings should not overlap.

// count11("11abc11") → 2
// count11("abc11x11x11") → 3
// count11("111") → 1

fn count11(s: &str) -> u32 {
    if s.len() < 2 {
        0
    } else {
        if &s[..2] == "11" {
            1 + count11(&s[2..])
        } else {
            0 + count11(&s[1..])
        }
    }
}

// Recursion-1 > parenBit
// https://codingbat.com/prob/p137918

// Given a string that contains a single pair of parenthesis,
// compute recursively a new string made of only of the parenthesis and their contents, so "xyz(abc)123" yields "(abc)".

// paren_bit("xyz(abc)123") → "(abc)"
// paren_bit("x(hello)") → "(hello)"
// paren_bit("(xy)1") → "(xy)"

fn paren_bit(s: &str) -> String {
    let len = s.len();
    let first = s.chars().next().unwrap();
    let last = s.chars().last().unwrap();
    if first == '(' && last == ')' {
        s.to_string()
    } else {
        if first != '(' && last != ')' {
            paren_bit(&s[1..len-1])
        } else if first != '(' {
            paren_bit(&s[1..])
        } else if last != ')' {
            paren_bit(&s[..len-1])
        } else {
            panic!("The if else-if block missed something!")
        }
    }
}

// Recursion-1 > strCopies
// https://codingbat.com/prob/p118182

// Given a string and a non-empty substring sub,
// compute recursively if at least n copies of sub appear in the string somewhere, possibly with overlapping.
// N will be non-negative.

// str_copies("catcowcat", "cat", 2) → true
// str_copies("catcowcat", "cow", 2) → false
// str_copies("catcowcat", "cow", 1) → true

fn str_copies(s: &str, sub: &str, n: u32) -> bool {
    let (slen, sublen) = (s.len(), sub.len());
    if slen < sublen {
        n == 0
    } else {
        if &s[..sublen] == sub {
            str_copies(&s[sublen..], sub, n-1)
        } else {
            str_copies(&s[1..], sub, n)
        }
    }
}

// Recursion-1 > fibonacci
// https://codingbat.com/prob/p120015

// The fibonacci sequence is a famous bit of mathematics, and it happens to have a recursive definition.
// The first two values in the sequence are 0 and 1 (essentially 2 base cases).
// Each subsequent value is the sum of the previous two values,
// so the whole sequence is: 0, 1, 1, 2, 3, 5, 8, 13, 21 and so on.
// Define a recursive fibonacci(n) method that returns the nth fibonacci number,
// with n=0 representing the start of the sequence.

// fibonacci(0) → 0
// fibonacci(1) → 1
// fibonacci(2) → 1

fn fibonacci(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n-2) + fibonacci(n-1)
    }
}

// Recursion-1 > sumDigits
// https://codingbat.com/prob/p163932

// Given a non-negative int n, return the sum of its digits recursively (no loops).
// Note that mod (%) by 10 yields the rightmost digit (126 % 10 is 6),
// while divide (/) by 10 removes the rightmost digit (126 / 10 is 12).

// sum_digits(126) → 9
// sum_digits(49) → 13
// sum_digits(12) → 3

fn sum_digits(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n % 10 + sum_digits(n / 10)
    }
}

// Recursion-1 > powerN
// https://codingbat.com/prob/p158888

// Given base and n that are both 1 or more, compute recursively (no loops) the value of base to the n power,
// so powerN(3, 2) is 9 (3 squared).

// power_n(3, 1) → 3
// power_n(3, 2) → 9
// power_n(3, 3) → 27

fn power_n(base: u32, n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        base * power_n(base, n-1)
    }
}

// Recursion-1 > changeXY
// https://codingbat.com/prob/p101372

// Given a string, compute recursively (no loops) a new string
// where all the lowercase 'x' chars have been changed to 'y' chars.

// change_xy("codex") → "codey"
// change_xy("xxhixx") → "yyhiyy"
// change_xy("xhixhix") → "yhiyhiy"

fn change_xy(s: &str) -> String {
    if s.len() == 0 {
        "".to_string()
    } else {
        let head = s.chars().next().unwrap();
        if head == 'x' {
            format!("y{}", change_xy(&s[1..]))
        } else {
            format!("{}{}", head, change_xy(&s[1..]))
        }
    }
}

// Recursion-1 > array6
// https://codingbat.com/prob/p108997

// Given an array of ints, compute recursively if the array contains a 6.
// We'll use the convention of considering only the part of the array that begins at the given index.
// In this way, a recursive call can pass index+1 to move down the array. The initial call will pass in index as 0.

// array6([1, 6, 4], 0) → true
// array6([1, 4], 0) → false
// array6([6], 0) → true

fn array6(a: &[i32], index: usize) -> bool {
    if index >= a.len() {
        false
    } else {
        if a[index] == 6 {
            true
        } else {
            array6(&a, index+1)
        }
    }
}

// Recursion-1 > allStar
// https://codingbat.com/prob/p183394

// Given a string, compute recursively a new string where all the adjacent chars are now separated by a "*".

// all_star("hello") → "h*e*l*l*o"
// all_star("abc") → "a*b*c"
// all_star("ab") → "a*b"

fn all_star(s: &str) -> String {
    if s.len() < 2 {
        s.to_string()
    } else {
        let head = s.chars().next().unwrap();
        format!("{}*{}", head, all_star(&s[1..]))
    }
}

// Recursion-1 > countPairs
// https://codingbat.com/prob/p154048

// We'll say that a "pair" in a string is two instances of a char separated by a char.
// So "AxA" the A's make a pair. Pair's can overlap, so "AxAxA" contains 3 pairs -- 2 for A and 1 for x.
// Recursively compute the number of pairs in the given string.

// count_pairs("axa") → 1
// count_pairs("axax") → 2
// count_pairs("axbx") → 1

fn count_pairs(s: &str) -> u32 {
    if s.len() < 3 {
        0
    } else {
        let bytes = s[..3].as_bytes();
        if bytes[0] == bytes[2] {
            1 + count_pairs(&s[1..])
        } else {
            0 + count_pairs(&s[1..])
        }
    }
}

// Recursion-1 > stringClean
// https://codingbat.com/prob/p104029

// Given a string, return recursively a "cleaned" string
// where adjacent chars that are the same have been reduced to a single char. So "yyzzza" yields "yza".

// string_clean("yyzzza") → "yza"
// string_clean("abbbcdd") → "abcd"
// string_clean("Hello") → "Helo"

fn string_clean(s: &str) -> String {

    fn helper(s: &str, previous: Option<char>) -> String {
        if s.len() == 0 {
            "".to_string()
        } else {
            let head = s.chars().next().unwrap();
            if Some(head) == previous {
                helper(&s[1..], previous)
            } else {
                format!("{}{}", head, helper(&s[1..], Some(head)))
            }
        }
    }
    helper(&s, None)
}

// Recursion-1 > nestParen
// https://codingbat.com/prob/p183174

// Given a string, return true if it is a nesting of zero or more pairs of parenthesis,
// like "(())" or "((()))". Suggestion: check the first and last chars, and then recur on what's inside them.

// nest_paren("(())") → true
// nest_paren("((()))") → true
// nest_paren("(((x))") → false

fn nest_paren(s: &str) -> bool {
    let slen = s.len();
    if slen == 0 {
        true
    } else {
        let first = s.chars().next().unwrap();
        let last = s.chars().last().unwrap();
        if first == '(' && last == ')' {
            nest_paren(&s[1..slen-1])
        } else {
            false
        }
    }
}

// Recursion-1 > strDist
// https://codingbat.com/prob/p195413

// Given a string and a non-empty substring sub,
// compute recursively the largest substring which starts and ends with sub and return its length.

// str_dist("catcowcat", "cat") → 9
// str_dist("catcowcat", "cow") → 3
// str_dist("cccatcowcatxx", "cat") → 9

fn str_dist(s: &str, sub: &str) -> usize {
    let (slen, sublen) = (s.len(), sub.len());
    if &s[..sublen] == sub && &s[slen-sublen..] == sub {
        slen
    } else {
        if &s[..sublen] != sub && &s[slen-sublen..] != sub {
            str_dist(&s[1..slen-1], sub)
        } else if &s[..sublen] != sub {
            str_dist(&s[1..slen], sub)
        } else if &s[slen-sublen..] != sub {
            str_dist(&s[..slen-1], sub)
        } else {
            panic!("The if-else if block didn't catch all cases.")
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

    #[test]
    fn array11_test() {
        assert_eq!(array11(&[1, 2, 11], 0), 1);
        assert_eq!(array11(&[11, 11], 0), 2);
        assert_eq!(array11(&[1, 2, 3, 4], 0), 0);
    }

    #[test]
    fn pair_star_test() {
        assert_eq!(pair_star("hello"), "hel*lo");
        assert_eq!(pair_star("xxyy"), "x*xy*y");
        assert_eq!(pair_star("aaaa"), "a*a*a*a");
    }

    #[test]
    fn count_abc_test() {
        assert_eq!(count_abc("abc"), 1);
        assert_eq!(count_abc("abcxxabc"), 2);
        assert_eq!(count_abc("abaxxaba"), 2);
    }

    #[test]
    fn count_hi2_test() {
        assert_eq!(count_hi2("ahixhi"), 1);
        assert_eq!(count_hi2("ahibhi"), 2);
        assert_eq!(count_hi2("xhixhi"), 0);
    }

    #[test]
    fn str_count_test() {
        assert_eq!(str_count("catcowcat", "cat"), 2);
        assert_eq!(str_count("catcowcat", "cow"), 1);
        assert_eq!(str_count("catcowcat", "dog"), 0);
    }

    #[test]
    fn bunny_ears_test() {
        assert_eq!(bunny_ears(0), 0);
        assert_eq!(bunny_ears(1), 2);
        assert_eq!(bunny_ears(2), 4);
    }

    #[test]
    fn triangle_test() {
        assert_eq!(triangle(0), 0);
        assert_eq!(triangle(1), 1);
        assert_eq!(triangle(2), 3);
    }

    #[test]
    fn count8_test() {
        assert_eq!(count8(8), 1);
        assert_eq!(count8(818), 2);
        assert_eq!(count8(8818), 4);
    }

    #[test]
    fn count_hi_test() {
        assert_eq!(count_hi("xxhixx"), 1);
        assert_eq!(count_hi("xhixhix"), 2);
        assert_eq!(count_hi("hi"), 1);
    }

    #[test]
    fn no_x_test() {
        assert_eq!(no_x("xaxb"), "ab");
        assert_eq!(no_x("abc"), "abc");
        assert_eq!(no_x("xx"), "");
    }

    #[test]
    fn array220_test() {
        assert_eq!(array220(&[1, 2, 20], 0), true);
        assert_eq!(array220(&[3, 30], 0), true);
        assert_eq!(array220(&[3], 0), false);
    }

    #[test]
    fn end_x_test() {
        assert_eq!(end_x("xxre"), "rexx");
        assert_eq!(end_x("xxhixx"), "hixxxx");
        assert_eq!(end_x("xhixhix"), "hihixxx");
    }

    #[test]
    fn count11_test() {
        assert_eq!(count11("11abc11"), 2);
        assert_eq!(count11("abc11x11x11"), 3);
        assert_eq!(count11("111"), 1);
    }

    #[test]
    fn paren_bit_test() {
        assert_eq!(paren_bit("xyz(abc)123"), "(abc)");
        assert_eq!(paren_bit("x(hello)"), "(hello)");
        assert_eq!(paren_bit("(xy)1"), "(xy)");
    }

    #[test]
    fn str_copies_test() {
        assert_eq!(str_copies("catcowcat", "cat", 2), true);
        assert_eq!(str_copies("catcowcat", "cow", 2), false);
        assert_eq!(str_copies("catcowcat", "cow", 1), true);
    }

    #[test]
    fn fibonacci_test() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(8), 21);
    }

    #[test]
    fn sum_digits_test() {
        assert_eq!(sum_digits(126), 9);
        assert_eq!(sum_digits(49), 13);
        assert_eq!(sum_digits(12), 3);
    }

    #[test]
    fn power_n_test() {
        assert_eq!(power_n(3, 1), 3);
        assert_eq!(power_n(3, 2), 9);
        assert_eq!(power_n(3, 3), 27);
    }

    #[test]
    fn change_xy_test() {
        assert_eq!(change_xy("codex"), "codey");
        assert_eq!(change_xy("xxhixx"), "yyhiyy");
        assert_eq!(change_xy("xhixhix"), "yhiyhiy");
    }

    #[test]
    fn array6_test() {
        assert_eq!(array6(&[1, 6, 4], 0), true);
        assert_eq!(array6(&[1, 4], 0), false);
        assert_eq!(array6(&[6], 0), true);
    }

    #[test]
    fn all_star_test() {
        assert_eq!(all_star("hello"), "h*e*l*l*o");
        assert_eq!(all_star("abc"), "a*b*c");
        assert_eq!(all_star("ab"), "a*b");
    }

    #[test]
    fn count_pairs_test() {
        assert_eq!(count_pairs("axa"), 1);
        assert_eq!(count_pairs("axax"), 2);
        assert_eq!(count_pairs("axbx"), 1);
    }

    #[test]
    fn string_clean_test() {
        assert_eq!(string_clean("yyzzza"), "yza");
        assert_eq!(string_clean("abbbcdd"), "abcd");
        assert_eq!(string_clean("Hello"), "Helo");
    }

    #[test]
    fn nest_paren_test() {
        assert_eq!(nest_paren("(())"), true);
        assert_eq!(nest_paren("((()))"), true);
        assert_eq!(nest_paren("(((x))"), false);
    }

    #[test]
    fn str_dist_test() {
        assert_eq!(str_dist("catcowcat", "cat"), 9);
        assert_eq!(str_dist("catcowcat", "cow"), 3);
        assert_eq!(str_dist("cccatcowcatxx", "cat"), 9);
    }
}
