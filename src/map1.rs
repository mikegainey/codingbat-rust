// I don't know how to get hash maps to work!!!


use std::collections::HashMap;

// Map-1 > mapBully
// https://codingbat.com/prob/p197888

// Modify and return the given map as follows:
// if the key "a" has a value, set the key "b" to have that value, and set the key "a" to have the value "".
// Basically "b" is a bully, taking the value and replacing it with the empty string.

// map_bully({"a": "candy", "b": "dirt"}) → {"a": "", "b": "candy"}
// map_bully({"a": "candy"}) → {"a": "", "b": "candy"}
// map_bully({"a": "candy", "b": "carrot", "c": "meh"}) → {"a": "", "b": "candy", "c": "meh"}

fn map_bully(map: &HashMap<String, String>) -> HashMap<String, String> {
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_bully_test() {
        // map_bully({"a": "candy", "b": "dirt"}) → {"a": "", "b": "candy"}
        let mut in_map = HashMap::new();
        in_map.insert("a".to_string(), "candy".to_string());
        in_map.insert("b".to_string(), "dirt".to_string());
        let out_map = map_bully(in_map);
        assert_eq!(&out_map.get("a").unwrap(), "".to_string());
        assert_eq!(&out_map.get("b").unwrap(), "candy".to_string());
        // map_bully({"a": "candy"}) → {"a": "", "b": "candy"}
        // map_bully({"a": "candy", "b": "carrot", "c": "meh"}) → {"a": "", "b": "candy", "c": "meh"}
    }
}
