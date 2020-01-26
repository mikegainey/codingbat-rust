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

// Logic-1 > caughtSpeeding
// https://codingbat.com/prob/p157733

// You are driving a little too fast, and a police officer stops you.
// Write code to compute the result, encoded as an int value: 0=no ticket, 1=small ticket, 2=big ticket.
// If speed is 60 or less, the result is 0.
// If speed is between 61 and 80 inclusive, the result is 1.
// If speed is 81 or more, the result is 2.
// Unless it is your birthday -- on that day, your speed can be 5 higher in all cases.

// caught_speeding(60, false) → 0
// caught_speeding(65, false) → 1
// caught_speeding(65, true) → 0

fn caught_speeding(speed: i32, is_birthday: bool) -> u8 {
    let birthday_waiver = if is_birthday {
        -5
    } else {
        0
    };
    if (speed + birthday_waiver) <= 60 {
        0
    } else if (speed + birthday_waiver) <= 80 {
        1
    } else {
        2
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

    #[test]
    fn caught_speeding_test() {
        assert_eq!(caught_speeding(60, false), 0);
        assert_eq!(caught_speeding(65, false), 1);
        assert_eq!(caught_speeding(65, true), 0);
    }
}
