// Warmup-2 > stringTimes
// https://codingbat.com/prob/p142270

// Given a string and a non-negative int n, return a larger string that is n copies of the original string.

// string_times("Hi", 2) → "HiHi"
// string_times("Hi", 3) → "HiHiHi"
// string_times("Hi", 1) → "Hi"

fn string_times(s: &str, n: usize) -> String {
    s.repeat(n)
}

// Warmup-2 > doubleX
// https://codingbat.com/prob/p186759

// Given a string, return true if the first instance of "x" in the string is immediately followed by another "x".

// double_x("axxbb") → true
// double_x("axaxax") → false
// double_x("xxxxx") → true

fn double_x(s: &str) -> bool {
    let mut previous_char = ' '; // this assumes the input string doesn't contain spaces

    for c in s.chars() {
        if (c == 'x') && (previous_char == 'x') {
            return true
        }
        previous_char = c;
    }
    false
}

// Warmup-2 > last2
// https://codingbat.com/prob/p178318

// Given a string, return the count of the number of times that a substring length 2 appears in the string
// and also as the last 2 chars of the string, so "hixxxhi" yields 1 (we won't count the end substring).

// last2("hixxhi") → 1
// last2("xaxxaxaxx") → 1
// last2("axxxaaxx") → 2

fn last2a(s: &str) -> u32 {
    let len = s.len();
    let substring = &s[len-2..];
    let search_string = &s[..len-2];

    // with a for loop
    let mut count = 0;
    for x in 0..=search_string.len()-2 {
        if &search_string[x..x+2] == substring {
            count += 1
        }
    }
    count
}

fn last2b(s: &str) -> u32 {
    let len = s.len();
    let substring = &s[len-2..];
    let search_string = &s[..len-2];

    // with a recursive helper function
    fn substring_count(search_string: &str, substring: &str) -> u32 {
        if search_string.len() < 2 {
            0
        } else {
            if &search_string[0..2] == substring {
                1 + substring_count(&search_string[1..], &substring)
            } else {
                0 + substring_count(&search_string[1..], &substring)
            }
        }
    }

    substring_count(&search_string, &substring)
}

// Warmup-2 > array123
// https://codingbat.com/prob/p136041

// Given an array of ints, return true if the sequence of numbers 1, 2, 3 appears in the array somewhere.

// array123([1, 1, 2, 3, 1]) → true
// array123([1, 1, 2, 4, 1]) → false
// array123([1, 1, 2, 1, 2, 3]) → true

fn array123a(a: &[i32]) -> bool {
    let len = a.len();
    for x in 0..=len-3 {
        if &a[x..x+3] == &[1,2,3] {
            return true
        }
    }
    false
}

fn array123b(a: &[i32]) -> bool {
    if a.len() < 3 {
        return false
    } else {
        return (&a[0..3] == &[1,2,3]) || array123b(&a[1..])
    }
}

// Warmup-2 > altPairs
// https://codingbat.com/prob/p121596

// Given a string, return a string made of the chars at indexes 0,1, 4,5, 8,9 ... so "kittens" yields "kien".

// alt_pairs("kitten") → "kien"
// alt_pairs("Chocolate") → "Chole"
// alt_pairs("CodingHorror") → "Congrr"

// with recursion
fn alt_pairs1(s: &str) -> String {
    if s.len() <= 2 {
        s.to_string()
    } else if s.len() <= 4 {
        format!("{}", &s[..2])
    } else {
        format!("{}{}", &s[..2], alt_pairs1(&s[4..]))
    }
}

// with iteration, pushing to accumulate an output variable
fn alt_pairs2(s: &str) -> String {
    let mut output = String::new();
    let mut mod4x;
    for (x, c) in s.chars().enumerate() {
        mod4x = x % 4;
        if mod4x == 0 || mod4x == 1 {
            output.push(c);
        }
    }
    output
}

// nice!  the rusty way with iterator adaptors
fn alt_pairs3(s: &str) -> String {
    s.chars()
        .enumerate()
        .filter(|(x,_)| x % 4 == 0 || x % 4 == 1)
        .map(|(_,c)| c)
        .collect::<String>()
}

// Warmup-2 > noTriples
// https://codingbat.com/prob/p170221

// Given an array of ints, we'll say that a triple is a value appearing 3 times in a row in the array.
// Return true if the array does not contain any triples.

// no_triples([1, 1, 2, 2, 1]) → true
// no_triples([1, 1, 2, 2, 2, 1]) → false
// no_triples([1, 1, 1, 2, 2, 2, 1]) → false

fn no_triples1(a: &[i32]) -> bool {
    let len = a.len();
    for x in 0..=len-3 {
        if a[x] == a[x+1] && a[x] == a[x+2] {
            return false
        }
    }
    true
}

fn no_triples2(a: &[i32]) -> bool {
    if a.len() < 3 {
        true
    } else {
        let no_triple = !(a[0] == a[1] && a[0] == a[2]);
        no_triple && no_triples2(&a[1..])
    }
}

// Warmup-2 > frontTimes
// https://codingbat.com/prob/p101475

// Given a string and a non-negative int n, // we'll say that the front of the string is the first 3 chars,
// or whatever is there if the string is less than length 3.
// Return n copies of the front;

// front_times("Chocolate", 2) → "ChoCho"
// front_times("Chocolate", 3) → "ChoChoCho"
// front_times("Abc", 3) → "AbcAbcAbc"

fn front_times(s: &str, n: usize) -> String {
    use std::cmp;

    let lastx = cmp::min(3, s.len());
    let front = &s[..lastx];

    front.repeat(n)
}

// Warmup-2 > stringBits
// https://codingbat.com/prob/p165666

// Given a string, return a new string made of every other char starting with the first, so "Hello" yields "Hlo".

// string_bits("Hello") → "Hlo"
// string_bits("Hi") → "H"
// string_bits("Heeololeo") → "Hello"

fn string_bits(s: &str) -> String {
    s.chars()
        .step_by(2)
        .collect::<String>()
}

// Warmup-2 > arrayCount9
// https://codingbat.com/prob/p184031

// Given an array of ints, return the number of 9's in the array.

// array_count9([1, 2, 9]) → 1
// array_count9([1, 9, 9]) → 2
// array_count9([1, 9, 9, 3, 9]) → 3

fn array_count9a(a: &[i32]) -> usize {
    a.iter()
        .filter(|&x| x == &9)
        .count()
}

fn array_count9b(a: &[i32]) -> usize {
    let mut count = 0;
    for x in a.iter() {
        if x == &9 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_times_test() {
        assert_eq!(string_times("Hi", 2), "HiHi");
        assert_eq!(string_times("Hi", 3), "HiHiHi");
        assert_eq!(string_times("Hi", 1), "Hi");
    }

    #[test]
    fn double_x_test() {
        assert_eq!(double_x("axxbb"), true);
        assert_eq!(double_x("axaxax"), false);
        assert_eq!(double_x("xxxxx"), true);
    }

    #[test]
    fn last2_test() {
        assert_eq!(last2a("hixxhi"), 1);
        assert_eq!(last2a("xaxxaxaxx"), 1);
        assert_eq!(last2a("axxxaaxx"), 2);
        assert_eq!(last2a("michmiaemimi"), 3);
        assert_eq!(last2b("hixxhi"), 1);
        assert_eq!(last2b("xaxxaxaxx"), 1);
        assert_eq!(last2b("axxxaaxx"), 2);
        assert_eq!(last2b("michmiaemimi"), 3);
    }

    #[test]
    fn array123_test() {
        assert_eq!(array123a(&[1, 1, 2, 3, 1]), true);
        assert_eq!(array123a(&[1, 1, 2, 4, 1]), false);
        assert_eq!(array123a(&[1, 1, 2, 1, 2, 3]), true);
        assert_eq!(array123a(&[1, 2, 3, 1, 2, 4]), true);
        assert_eq!(array123b(&[1, 1, 2, 3, 1]), true);
        assert_eq!(array123b(&[1, 1, 2, 4, 1]), false);
        assert_eq!(array123b(&[1, 1, 2, 1, 2, 3]), true);
        assert_eq!(array123b(&[1, 2, 3, 1, 2, 4]), true);
    }

    #[test]
    fn alt_pairs_test() {
        assert_eq!(alt_pairs1("kitten"), "kien");
        assert_eq!(alt_pairs1("Chocolate"), "Chole");
        assert_eq!(alt_pairs1("CodingHorror"), "Congrr");
        assert_eq!(alt_pairs2("kitten"), "kien");
        assert_eq!(alt_pairs2("Chocolate"), "Chole");
        assert_eq!(alt_pairs2("CodingHorror"), "Congrr");
        assert_eq!(alt_pairs3("kitten"), "kien");
        assert_eq!(alt_pairs3("Chocolate"), "Chole");
        assert_eq!(alt_pairs3("CodingHorror"), "Congrr");
    }

    #[test]
    fn no_triples_test() {
        assert_eq!(no_triples1(&[1, 1, 2, 2, 1]), true);
        assert_eq!(no_triples1(&[1, 1, 2, 2, 2, 1]), false);
        assert_eq!(no_triples1(&[1, 1, 1, 2, 2, 2, 1]), false);
        assert_eq!(no_triples2(&[1, 1, 2, 2, 1]), true);
        assert_eq!(no_triples2(&[1, 1, 2, 2, 2, 1]), false);
        assert_eq!(no_triples2(&[1, 1, 1, 2, 2, 2, 1]), false);
    }

    #[test]
    fn front_times_test() {
        assert_eq!(front_times("Chocolate", 2), "ChoCho");
        assert_eq!(front_times("Chocolate", 3), "ChoChoCho");
        assert_eq!(front_times("Abc", 3), "AbcAbcAbc");
    }

    #[test]
    fn string_bits_test() {
        assert_eq!(string_bits("Hello"), "Hlo");
        assert_eq!(string_bits("Hi"), "H");
        assert_eq!(string_bits("Heeololeo"), "Hello");
    }

    #[test]
    fn array_count9_test() {
        assert_eq!(array_count9a(&[1, 2, 9]), 1);
        assert_eq!(array_count9a(&[1, 9, 9]), 2);
        assert_eq!(array_count9a(&[1, 9, 9, 3, 9]), 3);
        assert_eq!(array_count9b(&[1, 2, 9]), 1);
        assert_eq!(array_count9b(&[1, 9, 9]), 2);
        assert_eq!(array_count9b(&[1, 9, 9, 3, 9]), 3);
    }
}
