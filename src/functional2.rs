// Functional-2 > noNeg
// https://codingbat.com/prob/p103456

// Given a list of integers, return a list of the integers, omitting any that are less than 0.

// no_neg([1, -2]) → [1]
// no_neg([-3, -3, 3, 3]) → [3, 3]
// no_neg([-1, -1, -1]) → []

fn no_neg(array: &[i32]) -> Vec<i32> {
    array.iter()
        .filter(|&&x| x >= 0) // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
        .cloned()             // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned
        .collect()
}

// Functional-2 > noZ
// https://codingbat.com/prob/p105671

// Given a list of strings, return a list of the strings, omitting any string that contains a "z".

// no_z(["aaa", "bbb", "aza"]) → ["aaa", "bbb"]
// no_z(["hziz", "hzello", "hi"]) → ["hi"]
// no_z(["hello", "howz", "are", "youz"]) → ["hello", "are"]

fn no_z<'a>(array: &[&'a str]) -> Vec<&'a str> { // added lifetimes until the compiler stopped complaining
    array.iter()
        .filter(|s| !s.contains("z"))
        .cloned()
        .collect()
}

// Functional-2 > noYY
// https://codingbat.com/prob/p115967

// Given a list of strings, return a list where each string has "y" added at its end,
// omitting any resulting strings that contain "yy" as a substring anywhere.

// no_yy(["a", "b", "c"]) → ["ay", "by", "cy"]
// no_yy(["a", "b", "cy"]) → ["ay", "by"]
// no_yy(["xx", "ya", "zz"]) → ["xxy", "yay", "zzy"]

fn no_yy(array: &[&str]) -> Vec<String> {
    array.iter()
        .map(|s| format!("{}y", s))
        .filter(|s| !s.contains("yy"))
        .collect()
}

// Functional-2 > no9
// https://codingbat.com/prob/p124510

// Given a list of non-negative integers, return a list of those numbers except omitting any that end with 9.

// no9([1, 2, 19]) → [1, 2]
// no9([9, 19, 29, 3]) → [3]
// no9([1, 2, 3]) → [1, 2, 3]

fn no9(array: &[i32]) -> Vec<i32> {
    array.iter()
        .filter(|&x| x % 10 != 9)
        .cloned()
        .collect()
}

// Functional-2 > noLong
// https://codingbat.com/prob/p194496

// Given a list of strings, return a list of the strings, omitting any string length 4 or more.

// no_long(["this", "not", "too", "long"]) → ["not", "too"]
// no_long(["a", "bbb", "cccc"]) → ["a", "bbb"]
// no_long(["cccc", "cccc", "cccc"]) → []

fn no_long<'a>(array: &[&'a str]) -> Vec<&'a str> {
    array.iter()
        .filter(|s| s.len() < 4)
        .cloned()
        .collect()
}

// Functional-2 > two2
// https://codingbat.com/prob/p148198

// Given a list of non-negative integers, return a list of those numbers multiplied by 2,
// omitting any of the resulting numbers that end in 2.

// two2([1, 2, 3]) → [4, 6]
// two2([2, 6, 11]) → [4]
// two2([0]) → [0]

fn two2(array: &[u32]) -> Vec<u32> {
    array.iter()
        .map(|x| x * 2)
        .filter(|x| x % 10 != 2)
        .collect()
}

// Functional-2 > noTeen
// https://codingbat.com/prob/p137274

// Given a list of integers, return a list of those numbers, omitting any that are between 13 and 19 inclusive.

// no_teen([12, 13, 19, 20]) → [12, 20]
// no_teen([1, 14, 1]) → [1, 1]
// no_teen([15]) → []

fn no_teen(array: &[i32]) -> Vec<i32> {
    array.iter()
        .filter(|&&x| x < 13 || x > 19)
        .cloned()
        .collect()
}

// Functional-2 > no34
// https://codingbat.com/prob/p184496

// Given a list of strings, return a list of the strings, omitting any string length 3 or 4.

// no34(["a", "bb", "ccc"]) → ["a", "bb"]
// no34(["a", "bb", "ccc", "dddd"]) → ["a", "bb"]
// no34(["ccc", "dddd", "apple"]) → ["apple"]

fn no34<'a>(array: &[&'a str]) -> Vec<&'a str> {
    array.iter()
        .filter(|s| s.len() != 3 && s.len() != 4)
        .cloned() // converts items from &&str to &str
        .collect()
}

// Functional-2 > square56
// https://codingbat.com/prob/p132748

// Given a list of integers, return a list of those numbers squared and the product added to 10,
// omitting any of the resulting numbers that end in 5 or 6.

// square56([3, 1, 4]) → [19, 11]
// square56([1]) → [11]
// square56([2]) → [14]

fn square56(array: &[i32]) -> Vec<i32> {
    array.iter()
        .map(|x| x * x + 10)
        .filter(|x| x % 10 != 5 && x % 10 != 6)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_neg_test() {
        assert_eq!(no_neg(&[1, -2]), [1]);
        assert_eq!(no_neg(&[-3, -3, 3, 3]), [3, 3]);
        assert_eq!(no_neg(&[-1, -1, -1]), []);
    }

    #[test]
    fn no_z_test() {
        assert_eq!(no_z(&["aaa", "bbb", "aza"]), ["aaa", "bbb"]);
        assert_eq!(no_z(&["hziz", "hzello", "hi"]), ["hi"]);
        assert_eq!(no_z(&["hello", "howz", "are", "youz"]), ["hello", "are"]);
    }

    #[test]
    fn no_yy_test() {
        assert_eq!(no_yy(&["a", "b", "c"]), ["ay", "by", "cy"]);
        assert_eq!(no_yy(&["a", "b", "cy"]), ["ay", "by"]);
        assert_eq!(no_yy(&["xx", "ya", "zz"]), ["xxy", "yay", "zzy"]);
    }

    #[test]
    fn no9_test() {
        assert_eq!(no9(&[1, 2, 19]), [1, 2]);
        assert_eq!(no9(&[9, 19, 29, 3]), [3]);
        assert_eq!(no9(&[1, 2, 3]), [1, 2, 3]);
    }

    #[test]
    fn no_long_test() {
        assert_eq!(no_long(&["this", "not", "too", "long"]), ["not", "too"]);
        assert_eq!(no_long(&["a", "bbb", "cccc"]), ["a", "bbb"]);
        // assert_eq!(no_long(&["cccc", "cccc", "cccc"]), []); // cannot infer type ??
    }

    #[test]
    fn two2_test() {
        assert_eq!(two2(&[1, 2, 3]), [4, 6]);
        assert_eq!(two2(&[2, 6, 11]), [4]);
        assert_eq!(two2(&[0]), [0]);
    }

    #[test]
    fn no_teen_test() {
        assert_eq!(no_teen(&[12, 13, 19, 20]), [12, 20]);
        assert_eq!(no_teen(&[1, 14, 1]), [1, 1]);
        assert_eq!(no_teen(&[15]), []);
    }

    #[test]
    fn no34_test() {
        assert_eq!(no34(&["a", "bb", "ccc"]), ["a", "bb"]);
        assert_eq!(no34(&["a", "bb", "ccc", "dddd"]), ["a", "bb"]);
        assert_eq!(no34(&["ccc", "dddd", "apple"]), ["apple"]);
    }

    #[test]
    fn square56_test() {
        assert_eq!(square56(&[3, 1, 4]), [19, 11]);
        assert_eq!(square56(&[1]), [11]);
        assert_eq!(square56(&[2]), [14]);
    }
}
