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
}
