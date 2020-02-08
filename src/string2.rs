// String-2 > doubleChar
// https://codingbat.com/prob/p165312

// Given a string, return a string where for every char in the original, there are two chars.

// double_char("The") → "TThhee"
// double_char("AAbb") → "AAAAbbbb"
// double_char("Hi-There") → "HHii--TThheerree"

fn double_char(s: &str) -> String {
    s.chars()
        .map(|c| format!("{0}{0}", c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_char_test() {
        assert_eq!(double_char("The"), "TThhee");
        assert_eq!(double_char("AAbb"), "AAAAbbbb");
        assert_eq!(double_char("Hi-There"), "HHii--TThheerree");
    }
}
