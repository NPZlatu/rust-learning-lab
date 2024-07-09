use std::collections::HashMap;

fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    let mapper: HashMap<char, char> = [
        ('(', ')'),
        ('{', '}'),
        ('[', ']'),
    ]
        .iter()
        .cloned()
        .collect();

    for ch in s.chars() {
        if mapper.contains_key(&ch) {
            stack.push(ch);
        } else if let Some(&top) = stack.last() {
            if mapper.get(&top) == Some(&ch) {
                stack.pop();
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}

fn main() {
    let test_string = String::from("()[]{}");
    println!("{}", is_valid(test_string)); // prints true
}
