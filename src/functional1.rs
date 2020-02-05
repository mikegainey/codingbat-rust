// Functional-1 > doubling
// https://codingbat.com/prob/p117665

// Given a list of integers, return a list where each integer is multiplied by 2.

// doubling([1, 2, 3]) → [2, 4, 6]
// doubling([6, 8, 6, 8, -1]) → [12, 16, 12, 16, -2]
// doubling([]) → []

fn doubling(array: &[i32]) -> Vec<i32> {
    array.iter()
        .map(|x| x * 2)
        .collect()
}

// Functional-1 > copies3
// https://codingbat.com/prob/p181634

// Given a list of strings, return a list where each string is replaced by 3 copies of the string concatenated together.

// copies3(["a", "bb", "ccc"]) → ["aaa", "bbbbbb", "ccccccccc"]
// copies3(["24", "a", ""]) → ["242424", "aaa", ""]
// copies3(["hello", "there"]) → ["hellohellohello", "theretherethere"]

fn copies3(array: &[&str]) -> Vec<String> {
    array.iter()
        .map(|s| s.repeat(3))
        .collect()
}

// Functional-1 > rightDigit
// https://codingbat.com/prob/p152194

// Given a list of non-negative integers, return an integer list of the rightmost digits. (Note: use %)

// right_digit([1, 22, 93]) → [1, 2, 3]
// right_digit([16, 8, 886, 8, 1]) → [6, 8, 6, 8, 1]
// right_digit([10, 0]) → [0, 0]

fn right_digit(array: &[i32]) -> Vec<i32> {
    array.iter()
        .map(|x| x % 10)
        .collect()
}

// Functional-1 > square
// https://codingbat.com/prob/p139586

// Given a list of integers, return a list where each integer is multiplied with itself.

// square([1, 2, 3]) → [1, 4, 9]
// square([6, 8, -6, -8, 1]) → [36, 64, 36, 64, 1]
// square([]) → []

fn square(array: &[i32]) -> Vec<i32> {
    array.iter()
        .map(|x| x * x)
        .collect()
}

// Functional-1 > moreY
// https://codingbat.com/prob/p177528

// Given a list of strings, return a list where each string has "y" added at its start and end.

// more_y(["a", "b", "c"]) → ["yay", "yby", "ycy"]
// more_y(["hello", "there"]) → ["yhelloy", "ytherey"]
// more_y(["yay"]) → ["yyayy"]

fn more_y(array: &[&str]) -> Vec<String> {
    array.iter()
        .map(|s| format!("y{}y", s))
        .collect()
}

// Functional-1 > lower
// https://codingbat.com/prob/p186894

// Given a list of strings, return a list where each string is converted to lower case (Note: String toLowerCase() method).

// lower(["Hello", "Hi"]) → ["hello", "hi"]
// lower(["AAA", "BBB", "ccc"]) → ["aaa", "bbb", "ccc"]
// lower(["KitteN", "ChocolaTE"]) → ["kitten", "chocolate"]

fn lower(array: &[&str]) -> Vec<String> {
    array.iter()
        .map(|s| s.to_lowercase())
        .collect()
}

// Functional-1 > addStar
// https://codingbat.com/prob/p170181

// Given a list of strings, return a list where each string has "*" added at its end.

// add_star(["a", "bb", "ccc"]) → ["a*", "bb*", "ccc*"]
// add_star(["hello", "there"]) → ["hello*", "there*"]
// add_star(["*"]) → ["**"]

fn add_star(array: &[&str]) -> Vec<String> {
    array.iter()
        .map(|s| format!("{}*", s))
        .collect()
}

// Functional-1 > math1
// https://codingbat.com/prob/p103869

// Given a list of integers, return a list where each integer is added to 1 and the result is multiplied by 10.

// math1([1, 2, 3]) → [20, 30, 40]
// math1([6, 8, 6, 8, 1]) → [70, 90, 70, 90, 20]
// math1([10]) → [110]

fn math1(array: &[i32]) -> Vec<i32> {
    array.iter()
        .map(|x| (x + 1) * 10)
        .collect()
}

// Functional-1 > noX
// https://codingbat.com/prob/p105967

// Given a list of strings, return a list where each string has all its "x" removed.

// no_x(["ax", "bb", "cx"]) → ["a", "bb", "c"]
// no_x(["xxax", "xbxbx", "xxcx"]) → ["a", "bb", "c"]
// no_x(["x"]) → [""]

fn no_x(array: &[&str]) -> Vec<String> {
    array.iter()
        .map(|&s| s.chars().filter(|&c| c != 'x').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubling_test() {
        assert_eq!(doubling(&[1, 2, 3]), [2, 4, 6]);
        assert_eq!(doubling(&[6, 8, 6, 8, -1]), [12, 16, 12, 16, -2]);
        assert_eq!(doubling(&[]), []);
    }

    #[test]
    fn copies3_test() {
        assert_eq!(copies3(&["a", "bb", "ccc"]), ["aaa", "bbbbbb", "ccccccccc"]);
        assert_eq!(copies3(&["24", "a", ""]), ["242424", "aaa", ""]);
        assert_eq!(copies3(&["hello", "there"]), ["hellohellohello", "theretherethere"]);
    }

    #[test]
    fn right_digit_test() {
        assert_eq!(right_digit(&[1, 22, 93]), [1, 2, 3]);
        assert_eq!(right_digit(&[16, 8, 886, 8, 1]), [6, 8, 6, 8, 1]);
        assert_eq!(right_digit(&[10, 0]), [0, 0]);
    }

    #[test]
    fn square_test() {
        assert_eq!(square(&[1, 2, 3]), [1, 4, 9]);
        assert_eq!(square(&[6, 8, -6, -8, 1]), [36, 64, 36, 64, 1]);
        assert_eq!(square(&[]), []);
    }

    #[test]
    fn more_y_test() {
        assert_eq!(more_y(&["a", "b", "c"]), ["yay", "yby", "ycy"]);
        assert_eq!(more_y(&["hello", "there"]), ["yhelloy", "ytherey"]);
        assert_eq!(more_y(&["yay"]), ["yyayy"]);
    }

    #[test]
    fn lower_test() {
        assert_eq!(lower(&["Hello", "Hi"]), ["hello", "hi"]);
        assert_eq!(lower(&["AAA", "BBB", "ccc"]), ["aaa", "bbb", "ccc"]);
        assert_eq!(lower(&["KitteN", "ChocolaTE"]), ["kitten", "chocolate"]);
    }

    #[test]
    fn add_star_test() {
        assert_eq!(add_star(&["a", "bb", "ccc"]), ["a*", "bb*", "ccc*"]);
        assert_eq!(add_star(&["hello", "there"]), ["hello*", "there*"]);
        assert_eq!(add_star(&["*"]), ["**"]);
    }

    #[test]
    fn math1_test() {
        assert_eq!(math1(&[1, 2, 3]), [20, 30, 40]);
        assert_eq!(math1(&[6, 8, 6, 8, 1]), [70, 90, 70, 90, 20]);
        assert_eq!(math1(&[10]), [110]);
    }

    #[test]
    fn no_x_test() {
        assert_eq!(no_x(&["ax", "bb", "cx"]), ["a", "bb", "c"]);
        assert_eq!(no_x(&["xxax", "xbxbx", "xxcx"]), ["a", "bb", "c"]);
        assert_eq!(no_x(&["x"]), [""]);
    }
}
