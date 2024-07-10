use std::collections::HashMap;

fn is_isomorphic(s: &str, t: &str) -> bool {
    let mut st_map: HashMap<char, char> = HashMap::new();

    for (i, char_s) in s.chars().enumerate() {
        if let Some(&mapped_char) = st_map.get(&char_s) {
            if mapped_char != t.chars().nth(i).unwrap() {
                return false;
            }
        } else {
            // Check if the value has already been mapped to another key
            if st_map.values().any(|&val| val == t.chars().nth(i).unwrap()) {
                return false;
            }
            st_map.insert(char_s, t.chars().nth(i).unwrap());
        }
    }

    true
}

fn main() {
    // Example usage:
    println!("{}", is_isomorphic("egg", "add")); // Output: true
    println!("{}", is_isomorphic("foo", "bar")); // Output: false
    println!("{}", is_isomorphic("paper", "title")); // Output: true
}
