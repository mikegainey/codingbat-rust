// Logic-1 > cigarParty
// https://codingbat.com/prob/p159531

// When squirrels get together for a party, they like to have cigars.
// A squirrel party is successful when the number of cigars is between 40 and 60, inclusive.
// Unless it is the weekend, in which case there is no upper bound on the number of cigars.
// Return true if the party with the given values is successful, or false otherwise.

// cigar_party(30, false) → false
// cigar_party(50, false) → true
// cigar_party(70, true) → true

fn cigar_party(cigars: u32, is_weekend: bool) -> bool {
    if is_weekend {
        cigars >= 40
    } else {
        cigars >= 40 && cigars <= 60
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cigar_party_test() {
    assert_eq!(cigar_party(30, false), false);
    assert_eq!(cigar_party(50, false), true);
    assert_eq!(cigar_party(70, true), true);
    }
}
