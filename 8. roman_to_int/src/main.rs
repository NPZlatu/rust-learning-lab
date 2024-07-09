use std::collections::HashMap;

fn roman_to_int(s: &str) -> i32 {
    let mut integer = 0;
    let map: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
        .iter()
        .cloned()
        .collect();

    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();
    let mut i = 0;

    while i < length {
        let value = if i + 1 < length && map[&chars[i + 1]] > map[&chars[i]] {
            map[&chars[i + 1]] - map[&chars[i]]
        } else {
            map[&chars[i]]
        };

        if i + 1 < length && map[&chars[i + 1]] > map[&chars[i]] {
            i += 1;
        }

        integer += value;
        i += 1;
    }

    integer
}

fn main() {
    let test_string = "MCMXCIV";
    println!("{}", roman_to_int(test_string)); // prints 1994
}
