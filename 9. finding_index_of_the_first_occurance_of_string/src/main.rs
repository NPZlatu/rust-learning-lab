fn str_str(haystack: &str, needle: &str) -> i32 {
    let mut j = 0;
    let mut index: i32 = -1;

    if needle.len() > haystack.len() {
        return index;
    }

    let haystack_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();

    let mut i = 0;
    while i < haystack_chars.len() {
        while i < haystack_chars.len() && haystack_chars[i] == needle_chars[j] {
            index = if index == -1 { i as i32 } else { index };
            i += 1;
            j += 1;

            if j == needle_chars.len() {
                return index;
            }
        }

        if index != -1 {
            i = index as usize;
            i += 1;
            j = 0;
            index = -1;
        } else {
            i += 1;
        }
    }

    -1
}

fn main() {
    let haystack = "mississippi";
    let needle = "pi";
    println!("{}", str_str(haystack, needle)); // Should print 9
}
