use std::collections::HashMap;

fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count_letter_mappers: HashMap<char, i32> = HashMap::new();

    for ch in s.chars() {
        let count = count_letter_mappers.entry(ch).or_insert(0);
        *count += 1;
    }

    for ch in t.chars() {
        let count = count_letter_mappers.entry(ch).or_insert(0);
        *count -= 1;
    }

    for &count in count_letter_mappers.values() {
        if count != 0 {
            return false;
        }
    }

    true
}

fn main() {
    let s = "anagram";
    let t = "nagaram";

    println!("{}", is_anagram(s, t)); // Should print true

    let s = "rat";
    let t = "car";

    println!("{}", is_anagram(s, t)); // Should print false
}
