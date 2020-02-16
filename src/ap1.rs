// AP-1 > scoresIncreasing
// https://codingbat.com/prob/p146974

// Given an array of scores, return true if each score is equal or greater than the one before.
// The array will be length 2 or more.

// scores_increasing([1, 3, 4]) → true
// scores_increasing([1, 3, 2]) → false
// scores_increasing([1, 1, 4]) → true

fn scores_increasing(array: &[i32]) -> bool {
    for index in 0..=array.len()-2 {
        if array[index] > array[index+1] {
            return false
        }
    }
    true
}

// AP-1 > scoresAverage
// https://codingbat.com/prob/p123837

// Given an array of scores, compute the int average of the first half and the second half, and return whichever is larger.
// We'll say that the second half begins at index length/2.
// The array length will be at least 2.
// To practice decomposition, write a separate helper method:

// scores_average([2, 2, 4, 4]) → 4
// scores_average([4, 4, 4, 2, 2, 2]) → 4
// scores_average([3, 4, 5, 1, 2, 3]) → 4

fn scores_average(array: &[i32]) -> i32 {

    fn average(array: &[i32]) -> i32 {
        let mut sum = 0;
        for a in array.iter() {
            sum += a;
        }
        sum / (array.len() as i32)
    }

    let alen = array.len();
    let first_half = &array[..alen/2];
    let second_half = &array[alen/2..];

    std::cmp::max(average(first_half), average(second_half))
}

// AP-1 > wordsWithoutList
// https://codingbat.com/prob/p183407

// Given an array of strings, return a new List (e.g. an ArrayList) where all the strings of the given length are omitted.

// words_without_list(["a", "bb", "b", "ccc"], 1) → ["bb", "ccc"]
// words_without_list(["a", "bb", "b", "ccc"], 3) → ["a", "bb", "b"]
// words_without_list(["a", "bb", "b", "ccc"], 4) → ["a", "bb", "b", "ccc"]

fn words_without_list<'a>(array: &[&'a str], len: usize) -> Vec<&'a str> {
    array.iter()
        .filter(|w| w.len() != len)
        .cloned() // I don't know why this is needed!
        .collect()
}

// AP-1 > copyEvens
// https://codingbat.com/prob/p134174

// Given an array of positive ints,
// return a new array of length "count" containing the first even numbers from the original array.
// The original array will contain at least "count" even numbers.

// copy_evens([3, 2, 4, 5, 8], 2) → [2, 4]
// copy_evens([3, 2, 4, 5, 8], 3) → [2, 4, 8]
// copy_evens([6, 1, 2, 4, 5, 8], 3) → [6, 2, 4]

fn copy_evens(array: &[i32], count: usize) -> Vec<i32> {
    let mut output = Vec::new();
    for &a in array.iter() {
        if a % 2 == 0 {
            output.push(a);
        }
        if output.len() == count {
            break;
        }
    }
    output
}

// AP-1 > scoreUp
// https://codingbat.com/prob/p180365

// The "key" array is an array containing the correct answers to an exam, like {"a", "a", "b", "b"}.
// The "answers" array contains a student's answers, with "?" representing a question left blank.
// The two arrays are not empty and are the same length.
// Return the score for this array of answers,
// giving +4 for each correct answer, -1 for each incorrect answer, and +0 for each blank answer.

// score_up(["a", "a", "b", "b"], ["a", "c", "b", "c"]) → 6
// score_up(["a", "a", "b", "b"], ["a", "a", "b", "c"]) → 11
// score_up(["a", "a", "b", "b"], ["a", "a", "b", "b"]) → 16

fn score_up(key: &[&str], answers: &[&str]) -> i32 {
    key.iter()
        .zip(answers.iter())
        .map(|(&k, &a) | if a == k {
            4
        } else if a == "?" {
            0
        } else {
            -1
        })
        .sum()
}

// AP-1 > sumHeights
// https://codingbat.com/prob/p148138

// We have an array of heights, representing the altitude along a walking trail.
// Given start/end indexes into the array,
// return the sum of the changes for a walk beginning at the start index and ending at the end index.
// For example, with the heights {5, 3, 6, 7, 2} and start=2, end=4 yields a sum of 1 + 5 = 6.
// The start end end index will both be valid indexes into the array with start <= end.

// sum_heights([5, 3, 6, 7, 2], 2, 4) → 6
// sum_heights([5, 3, 6, 7, 2], 0, 1) → 2
// sum_heights([5, 3, 6, 7, 2], 0, 4) → 11

fn sum_heights(heights: &[i32], start: usize, end: usize) -> i32 {
    let mut total_changes = 0;
    for index in start..=end-1 {
        total_changes += (heights[index] - heights[index+1]).abs();
    }
    total_changes
}

// AP-1 > userCompare
// https://codingbat.com/prob/p143482

// We have data for two users, A and B, each with a String name and an int id.
// The goal is to order the users such as for sorting.
// Return -1 if A comes before B, 1 if A comes after B, and 0 if they are the same.
// Order first by the string names, and then by the id numbers if the names are the same.

// user_compare("bb", 1, "zz", 2) → -1
// user_compare("bb", 1, "aa", 2) → 1
// user_compare("bb", 1, "bb", 1) → 0

fn user_compare(a_name: &str, a_id: u32, b_name: &str, b_id: u32) -> i32 {
    use std::cmp::Ordering;
    let ordering = (a_name, a_id).cmp(&(b_name, b_id));
    match ordering {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

// AP-1 > scores100
// https://codingbat.com/prob/p179487

// Given an array of scores, return true if there are scores of 100 next to each other in the array.
// The array length will be at least 2.

// scores100([1, 100, 100]) → true
// scores100([1, 100, 99, 100]) → false
// scores100([100, 1, 100, 100]) → true

fn scores100(array: &[u32]) -> bool {
    for (&l, &r) in array.iter().zip(array[1..].iter()) {
        if l == 100 && r == 100 {
            return true;
        }
    }
    false
}

// AP-1 > wordsCount
// https://codingbat.com/prob/p124620

// Given an array of strings, return the count of the number of strings with the given length.

// words_count(["a", "bb", "b", "ccc"], 1) → 2
// words_count(["a", "bb", "b", "ccc"], 3) → 1
// words_count(["a", "bb", "b", "ccc"], 4) → 0

fn words_count(array: &[&str], len: usize) -> usize {
    array.iter()
        .filter(|&w| w.len() == len)
        .count() as usize
}

// AP-1 > hasOne
// https://codingbat.com/prob/p191212

// Given a positive int n, return true if it contains a 1 digit.

// has_one(10) → true
// has_one(22) → false
// has_one(220) → false

fn has_one(mut n: u32) -> bool {
    while n > 0 {
        let (left, digit) = (n / 10, n % 10);
        if digit == 1 {
            return true;
        }
        n = left;
    }
    false
}

// AP-1 > copyEndy
// https://codingbat.com/prob/p130124

// We'll say that a positive int n is "endy" if it is in the range 0..10 or 90..100 (inclusive). Given an array of positive ints, return a new array of length "count" containing the first endy numbers from the original array. Decompose out a separate isEndy(int n) method to test if a number is endy. The original array will contain at least "count" endy numbers.

// copy_endy([9, 11, 90, 22, 6], 2) → [9, 90]
// copy_endy([9, 11, 90, 22, 6], 3) → [9, 90, 6]
// copy_endy([12, 1, 1, 13, 0, 20], 2) → [1, 1]

fn copy_endy(array: &[i32], count: usize) -> Vec<i32> {

    fn is_endy(n: i32) -> bool {
        (n >= 0 && n <= 10) || (n >= 90 && n <= 100)
    }

    let mut output = Vec::new();
    for &a in array.iter() {
        if is_endy(a) {
            output.push(a);
        }
        if output.len() == count {
            break;
        }
    }
    output
}

// AP-1 > wordsWithout
// https://codingbat.com/prob/p121236

// Given an array of strings, return a new array without the strings that are equal to the target string.
// One approach is to count the occurrences of the target string, make a new array of the correct length,
// and then copy over the correct strings.

// words_without(["a", "b", "c", "a"], "a") → ["b", "c"]
// words_without(["a", "b", "c", "a"], "b") → ["a", "c", "a"]
// words_without(["a", "b", "c", "a"], "c") → ["a", "b", "a"]

fn words_without<'a>(words: &[&'a str], target: &str) -> Vec<&'a str> {
    words.iter()
        .filter(|&&w| w != target)
        .cloned() // I don't know why this is needed!
        .collect()

    // hypothesis: The items returned from an iter are references, on top of the fact that
    // the values of the array already are references (&str) -- so they're returned as &&str.
}

// AP-1 > sumHeights2
// https://codingbat.com/prob/p157900

// We have an array of heights, representing the altitude along a walking trail.
// Given start/end indexes into the array,
// return the sum of the changes for a walk beginning at the start index and ending at the end index,
// however increases in height count double.
// For example, with the heights {5, 3, 6, 7, 2} and start=2, end=4 yields a sum of 1*2 + 5 = 7.
// The start end end index will both be valid indexes into the array with start <= end.

// sum_heights2([5, 3, 6, 7, 2], 2, 4) → 7
// sum_heights2([5, 3, 6, 7, 2], 0, 1) → 2
// sum_heights2([5, 3, 6, 7, 2], 0, 4) → 15

fn sum_heights2(heights: &[i32], start: usize, end: usize) -> i32 {
    let mut total_sum = 0;
    for index in start..=end-1 {
        if heights[index+1] > heights[index] {
            total_sum += (heights[index] - heights[index+1]).abs() * 2
        } else {
            total_sum += (heights[index] - heights[index+1]).abs()
        }
    }
    total_sum
}

// AP-1 > mergeTwo
// https://codingbat.com/prob/p139150

// Start with two arrays of strings, A and B, each with its elements in alphabetical order and without duplicates.
// Return a new array containing the first N elements from the two arrays.
// The result array should be in alphabetical order and without duplicates.
// A and B will both have a length which is N or more.
// The best "linear" solution makes a single pass over A and B,
// taking advantage of the fact that they are in alphabetical order, copying elements directly to the new array.

// merge_two(["a", "c", "z"], ["b", "f", "z"], 3) → ["a", "b", "c"]
// merge_two(["a", "c", "z"], ["c", "f", "z"], 3) → ["a", "c", "f"]
// merge_two(["f", "g", "z"], ["c", "f", "g"], 3) → ["c", "f", "g"]

fn merge_two<'a>(a: &[&'a str], b: &[&'a str], n: usize) -> Vec<&'a str> {
    let mut output = Vec::new();
    let mut ax = 0;
    let mut bx = 0;
    while output.len() < n {
        if a[ax] == b[bx] {
            output.push(a[ax]);
            ax += 1;
            bx += 1;
        } else if a[ax] < b[bx] {
            output.push(a[ax]);
            ax += 1;
        } else if b[bx] < a[ax] {
            output.push(b[bx]);
            bx += 1;
        }
    }
    output
}

// AP-1 > scoresClump
// https://codingbat.com/prob/p194530

// Given an array of scores sorted in increasing order,
// return true if the array contains 3 adjacent scores that differ from each other by at most 2,
// such as with {3, 4, 5} or {3, 5, 5}.

// scores_clump([3, 4, 5]) → true
// scores_clump([3, 4, 6]) → false
// scores_clump([1, 3, 5, 5]) → true

fn scores_clump(scores: &[i32]) -> bool {
    for index in 0..=scores.len()-3 {
        if (scores[index] - scores[index+1]).abs() <= 2 &&
            (scores[index] - scores[index+2]).abs() <= 2 &&
            (scores[index+1] - scores[index+2]).abs() <= 2 {
                return true;
            }
    }
    false
}

// AP-1 > wordsFront
// https://codingbat.com/prob/p183837

// Given an array of strings, return a new array containing the first N strings.
// N will be in the range 1..length.

// words_front(["a", "b", "c", "d"], 1) → ["a"]
// words_front(["a", "b", "c", "d"], 2) → ["a", "b"]
// words_front(["a", "b", "c", "d"], 3) → ["a", "b", "c"]

fn words_front<'a>(words: &[&'a str], n: usize) -> Vec<&'a str> {
    words[..n].to_vec()
}

// AP-1 > dividesSelf
// https://codingbat.com/prob/p165941

// We'll say that a positive int divides itself if every digit in the number divides into the number evenly.
// So for example 128 divides itself since 1, 2, and 8 all divide into 128 evenly.
// We'll say that 0 does not divide into anything evenly, so no number with a 0 digit divides itself.
// Note: use % to get the rightmost digit, and / to discard the rightmost digit.

// divides_self(128) → true
// divides_self(12) → true
// divides_self(120) → false

fn divides_self(n: u32) -> bool {
    let mut mut_n = n;
    let mut digit;
    while mut_n > 0 {
        digit = mut_n % 10;
        if digit == 0 || n % digit != 0 {
            return false;
        }
        mut_n = mut_n / 10;
    }
    true
}

// AP-1 > matchUp
// https://codingbat.com/prob/p139677

// Given 2 arrays that are the same length containing strings,
// compare the 1st string in one array to the 1st string in the other array, the 2nd to the 2nd and so on.
// Count the number of times that the 2 strings are non-empty and start with the same char.
// The strings may be any length, including 0.

// match_up(["aa", "bb", "cc"], ["aaa", "xx", "bb"]) → 1
// match_up(["aa", "bb", "cc"], ["aaa", "b", "bb"]) → 2
// match_up(["aa", "bb", "cc"], ["", "", "ccc"]) → 1

fn match_up(a: &[&str], b: &[&str]) -> usize {
    a.iter().zip(b.iter())
        .filter(|(&aword, &bword)|
                aword.len() > 0 &&
                bword.len() > 0 &&
                aword[..1] == bword[..1]
        )
        .count()
}

// AP-1 > scoresSpecial
// https://codingbat.com/prob/p140485

// Given two arrays, A and B, of non-negative int scores.
// A "special" score is one which is a multiple of 10, such as 40 or 90.
// Return the sum of largest special score in A and the largest special score in B.
// To practice decomposition, write a separate helper method which finds the largest special score in an array.

// scores_special([12, 10, 4], [2, 20, 30]) → 40
// scores_special([20, 10, 4], [2, 20, 10]) → 40
// scores_special([12, 11, 4], [2, 20, 31]) → 20

fn scores_special(a: &[u32], b: &[u32]) -> u32 {

    fn special_score(a: &[u32]) -> &u32 {
        a.iter()
            .filter(|&s| s % 10 == 0)
            .max().unwrap_or(&0)
    }

    special_score(a) + special_score(b)
}

// AP-1 > bigHeights
// https://codingbat.com/prob/p197710

// We have an array of heights, representing the altitude along a walking trail.
// Given start/end indexes into the array,
// return the number of "big" steps for a walk starting at the start index and ending at the end index.
// We'll say that step is big if it is 5 or more up or down.
// The start end end index will both be valid indexes into the array with start <= end.

// big_heights([5, 3, 6, 7, 2], 2, 4) → 1
// big_heights([5, 3, 6, 7, 2], 0, 1) → 0
// big_heights([5, 3, 6, 7, 2], 0, 4) → 1

fn big_heights(heights: &[i32], start: usize, end: usize) -> u32 {
    let mut big_steps = 0;
    for index in start..=end-1 {
        if (heights[index] - heights[index+1]).abs() >= 5 {
            big_steps += 1;
        }
    }
    big_steps
}

// AP-1 > commonTwo
// https://codingbat.com/prob/p100369

// Start with two arrays of strings, a and b, each in alphabetical order, possibly with duplicates.
// Return the count of the number of strings which appear in both arrays.
// The best "linear" solution makes a single pass over both arrays,
// taking advantage of the fact that they are in alphabetical order.

// common_two(["a", "c", "x"], ["b", "c", "d", "x"]) → 2
// common_two(["a", "c", "x"], ["a", "b", "c", "x", "z"]) → 3
// common_two(["a", "b", "c"], ["a", "b", "c"]) → 3

// I don't know if this handles duplicates correctly. The tests don't cover it.
fn common_two(a: &[&str], b: &[&str]) -> u32 {
    if a.len() == 0 || b.len() == 0 {
        0
    } else {
        if a[0] == b[0] {
            1 + common_two(&a[1..], &b[1..])
        } else if a[0] < b[0] {
            0 + common_two(&a[1..], b)
        } else if b[0] < a[0] {
            0 + common_two(a, &b[1..])
        } else {
            panic!("all cases were not covered.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scores_increasing_test() {
        assert_eq!(scores_increasing(&[1, 3, 4]), true);
        assert_eq!(scores_increasing(&[1, 3, 2]), false);
        assert_eq!(scores_increasing(&[1, 1, 4]), true);
    }

    #[test]
    fn scores_average_test() {
        assert_eq!(scores_average(&[2, 2, 4, 4]), 4);
        assert_eq!(scores_average(&[4, 4, 4, 2, 2, 2]), 4);
        assert_eq!(scores_average(&[3, 4, 5, 1, 2, 3]), 4);
    }

    #[test]
    fn words_without_list_test() {
        assert_eq!(words_without_list(&["a", "bb", "b", "ccc"], 1), ["bb", "ccc"]);
        assert_eq!(words_without_list(&["a", "bb", "b", "ccc"], 3), ["a", "bb", "b"]);
        assert_eq!(words_without_list(&["a", "bb", "b", "ccc"], 4), ["a", "bb", "b", "ccc"]);
    }

    #[test]
    fn copy_evens_test() {
        assert_eq!(copy_evens(&[3, 2, 4, 5, 8], 2), [2, 4]);
        assert_eq!(copy_evens(&[3, 2, 4, 5, 8], 3), [2, 4, 8]);
        assert_eq!(copy_evens(&[6, 1, 2, 4, 5, 8], 3), [6, 2, 4]);
    }

    #[test]
    fn score_up_test() {
        assert_eq!(score_up(&["a", "a", "b", "b"], &["a", "c", "b", "c"]), 6);
        assert_eq!(score_up(&["a", "a", "b", "b"], &["a", "a", "b", "c"]), 11);
        assert_eq!(score_up(&["a", "a", "b", "b"], &["a", "a", "b", "b"]), 16);
    }

    #[test]
    fn sum_heights_test() {
        assert_eq!(sum_heights(&[5, 3, 6, 7, 2], 2, 4), 6);
        assert_eq!(sum_heights(&[5, 3, 6, 7, 2], 0, 1), 2);
        assert_eq!(sum_heights(&[5, 3, 6, 7, 2], 0, 4), 11);
    }

    #[test]
    fn user_compare_test() {
        assert_eq!(user_compare("bb", 1, "zz", 2), -1);
        assert_eq!(user_compare("bb", 1, "aa", 2), 1);
        assert_eq!(user_compare("bb", 1, "bb", 1), 0);
        assert_eq!(user_compare("bb", 1, "bb", 2), -1);
        assert_eq!(user_compare("bb", 2, "bb", 1), 1);
    }

    #[test]
    fn scores100_test() {
        assert_eq!(scores100(&[1, 100, 100]), true);
        assert_eq!(scores100(&[1, 100, 99, 100]), false);
        assert_eq!(scores100(&[100, 1, 100, 100]), true);
        assert_eq!(scores100(&[100, 100, 99, 100]), true);
        assert_eq!(scores100(&[99, 100, 100, 99]), true);
    }

    #[test]
    fn words_count_test() {
        assert_eq!(words_count(&["a", "bb", "b", "ccc"], 1), 2);
        assert_eq!(words_count(&["a", "bb", "b", "ccc"], 3), 1);
        assert_eq!(words_count(&["a", "bb", "b", "ccc"], 4), 0);
    }

    #[test]
    fn has_one_test() {
        assert_eq!(has_one(10), true);
        assert_eq!(has_one(22), false);
        assert_eq!(has_one(220), false);
    }

    #[test]
    fn copy_endy_test() {
        assert_eq!(copy_endy(&[9, 11, 90, 22, 6], 2), [9, 90]);
        assert_eq!(copy_endy(&[9, 11, 90, 22, 6], 3), [9, 90, 6]);
        assert_eq!(copy_endy(&[12, 1, 1, 13, 0, 20], 2), [1, 1]);
    }

    #[test]
    fn words_without_test() {
        assert_eq!(words_without(&["a", "b", "c", "a"], "a"), ["b", "c"]);
        assert_eq!(words_without(&["a", "b", "c", "a"], "b"), ["a", "c", "a"]);
        assert_eq!(words_without(&["a", "b", "c", "a"], "c"), ["a", "b", "a"]);
    }

    #[test]
    fn sum_heights2_test() {
        assert_eq!(sum_heights2(&[5, 3, 6, 7, 2], 2, 4), 7);
        assert_eq!(sum_heights2(&[5, 3, 6, 7, 2], 0, 1), 2);
        assert_eq!(sum_heights2(&[5, 3, 6, 7, 2], 0, 4), 15);
    }

    #[test]
    fn merge_two_test() {
        assert_eq!(merge_two(&["a", "c", "z"], &["b", "f", "z"], 3), ["a", "b", "c"]);
        assert_eq!(merge_two(&["a", "c", "z"], &["c", "f", "z"], 3), ["a", "c", "f"]);
        assert_eq!(merge_two(&["f", "g", "z"], &["c", "f", "g"], 3), ["c", "f", "g"]);
    }

    #[test]
    fn scores_clump_test() {
        assert_eq!(scores_clump(&[3, 4, 5]), true);
        assert_eq!(scores_clump(&[3, 4, 6]), false);
        assert_eq!(scores_clump(&[1, 3, 5, 5]), true);
    }

    #[test]
    fn words_front_test() {
        assert_eq!(words_front(&["a", "b", "c", "d"], 1), ["a"]);
        assert_eq!(words_front(&["a", "b", "c", "d"], 2), ["a", "b"]);
        assert_eq!(words_front(&["a", "b", "c", "d"], 3), ["a", "b", "c"]);
    }

    #[test]
    fn divides_self_test() {
        assert_eq!(divides_self(128), true);
        assert_eq!(divides_self(12), true);
        assert_eq!(divides_self(120), false);
    }

    #[test]
    fn match_up_test() {
        assert_eq!(match_up(&["aa", "bb", "cc"], &["aaa", "xx", "bb"]), 1);
        assert_eq!(match_up(&["aa", "bb", "cc"], &["aaa", "b", "bb"]), 2);
        assert_eq!(match_up(&["aa", "bb", "cc"], &["", "", "ccc"]), 1);
    }

    #[test]
    fn special_scores_test() {
        assert_eq!(scores_special(&[12, 10, 4], &[2, 20, 30]), 40);
        assert_eq!(scores_special(&[20, 10, 4], &[2, 20, 10]), 40);
        assert_eq!(scores_special(&[12, 11, 4], &[2, 20, 31]), 20);
    }

    #[test]
    fn big_heights_test() {
        assert_eq!(big_heights(&[5, 3, 6, 7, 2], 2, 4), 1);
        assert_eq!(big_heights(&[5, 3, 6, 7, 2], 0, 1), 0);
        assert_eq!(big_heights(&[5, 3, 6, 7, 2], 0, 4), 1);
    }

    #[test]
    fn common_two_test() {
        assert_eq!(common_two(&["a", "c", "x"], &["b", "c", "d", "x"]), 2);
        assert_eq!(common_two(&["a", "c", "x"], &["a", "b", "c", "x", "z"]), 3);
        assert_eq!(common_two(&["a", "b", "c"], &["a", "b", "c"]), 3);
    }
}
