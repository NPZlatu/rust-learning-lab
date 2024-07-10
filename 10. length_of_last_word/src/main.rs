fn length_of_last_word(s: &str) -> usize {
    let mut i = (s.len() as isize) - 1;
    let mut length = 0;
    let s_bytes = s.as_bytes();

    // Trim trailing spaces
    while i >= 0 && s_bytes[i as usize] == b' ' {
        i -= 1;
    }

    // Calculate the length of the last word
    while i >= 0 && s_bytes[i as usize] != b' ' {
        length += 1;
        i -= 1;
    }

    length
}

fn main() {
    let s = "Hello, world  ";
    let length = length_of_last_word(s);
    println!("The length of the last word is: {}", length);
}
