// String-3 > countYZ
// https://codingbat.com/prob/p199171

// Given a string, count the number of words ending in 'y' or 'z'
// -- so the 'y' in "heavy" and the 'z' in "fez" count, but not the 'y' in "yellow" (not case sensitive).
// We'll say that a y or z is at the end of a word if there is not an alphabetic letter immediately following it.

// count_yz("fez day") → 2
// count_yz("day fez") → 2
// count_yz("day fyyyz") → 2

fn count_yz(s: &str) -> usize {
    let words: Vec<&str> = s.split(' ').collect();
    words.iter()
        .filter(|w|
                w.chars().last().unwrap() == 'y' ||
                w.chars().last().unwrap() == 'z')
        .count()
}

// String-3 > gHappy
// https://codingbat.com/prob/p198664

// We'll say that a lowercase 'g' in a string is "happy" if there is another 'g' immediately to its left or right.
// Return true if all the g's in the given string are happy.

// g_happy("xxggxx") → true
// g_happy("xxgxx") → false
// g_happy("xxggyygxx") → false

// fails on a lone g at the end

fn g_happy(s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        if c == 'g' {
            // count consecutive 'g's
            count += 1;
        } else {
            // check if non-g's follow a lone g
            if count == 1 {
                return false;
            } else {
                count = 0;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_yz_test() {
        assert_eq!(count_yz("fez day"), 2);
        assert_eq!(count_yz("day fez"), 2);
        assert_eq!(count_yz("day fyyyz"), 2);
        assert_eq!(count_yz("dax fyyyz"), 1);
        assert_eq!(count_yz("dax fyyya"), 0);
    }

    #[test]
    fn g_happy_test() {
        assert_eq!(g_happy("xxggxx"), true);
        assert_eq!(g_happy("xxgxx"), false);
        assert_eq!(g_happy("xxggyygxx"), false);
        assert_eq!(g_happy("xxgggyygxx"), false);
        assert_eq!(g_happy("xxggxxg"), false); // fail!
    }
}
